import { collection_path } from "../globals.ts";
import { getFileInfo } from "../utils/file_utils.ts";

const exclusions: string[] = [
  ".gitattributes",
  ".vscode",
  "LICENSE",
  "readme.md",
  ".git",
];

function processDirSearch(
  Entry: Deno.DirEntry,
  dirMode: boolean,
  searchEntry: string,
): number | null {
  if (dirMode && !Entry.isDirectory) {
    return null;
  } else if (!dirMode && Entry.isDirectory) {
    return null;
  }

  if (exclusions.includes(Entry.name)) {
    return null;
  }

  // If the search entry is empty, return a perfect match score of 1
  if (searchEntry.trim() === "") {
    return 1;
  }

  // search logic her
  const justFileName = Entry.name.split(".");

  const entryName = justFileName[0].toLocaleLowerCase();
  const normalizedSearchEntry = searchEntry.toLowerCase();

  if (entryName.includes(normalizedSearchEntry)) {
    return 1;
  }

  const score = calculateSimilarity(entryName, normalizedSearchEntry);

  return score;
}

export async function dirSearch_api_handler(
  _req: Request,
  url: URL,
): Promise<Response> {
  const dirPath = `${collection_path}/${url.pathname.substring(11)}`;

  // safety logic
  {
    const [fileExists, _fileInfo] = await getFileInfo(dirPath);

    if (!fileExists) {
      return new Response(null, { status: 404 });
    }

    if (!_fileInfo?.isDirectory) {
      return new Response("PATH IS A FILE\nConsider using /files API", {
        status: 400,
      });
    }
  }

  // searching logic
  {
    const directoryMode = url.searchParams.get("directoryMode") == "true";
    const searchEntry = url.searchParams.get("searchEntry") || "";
    const dirRead = Deno.readDir(dirPath);
    const results: string[] = [];

    for await (const dirEntry of dirRead) {
      const similarity = processDirSearch(dirEntry, directoryMode, searchEntry);
      if (similarity == null) continue;

      if (similarity >= 0.4) {
        results.push(dirEntry.name);
      }

      if (results.length >= 50) {
        break;
      }
    }

    return new Response(JSON.stringify(results), {
      status: 200,
      headers: {
        "Content-Type": "application/json",
        "Cache-Control": "no-cache"
      },
    });
  }
}

// similarity detection
function levenshteinDistance(a: string, b: string): number {
  const matrix: number[][] = [];

  // Initialize the matrix
  for (let i = 0; i <= a.length; i++) {
    matrix[i] = [i];
  }
  for (let j = 0; j <= b.length; j++) {
    matrix[0][j] = j;
  }

  // Populate the matrix
  for (let i = 1; i <= a.length; i++) {
    for (let j = 1; j <= b.length; j++) {
      const cost = a[i - 1] === b[j - 1] ? 0 : 1;
      matrix[i][j] = Math.min(
        matrix[i - 1][j] + 1, // Deletion
        matrix[i][j - 1] + 1, // Insertion
        matrix[i - 1][j - 1] + cost, // Substitution
      );
    }
  }

  return matrix[a.length][b.length];
}

function calculateSimilarity(entryName: string, searchEntry: string): number {
  const distance = levenshteinDistance(
    entryName.toLowerCase(),
    searchEntry.toLowerCase(),
  );
  const maxLength = Math.max(entryName.length, searchEntry.length);

  // Prevent division by zero and calculate similarity
  return maxLength === 0 ? 1 : 1 - distance / maxLength;
}
