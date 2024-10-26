import { collection_path } from "../globals.ts";
import { getFileInfo } from "../utils/file_utils.ts";

export async function frontmatter_api_handler(
  _req: Request,
  url: URL
): Promise<Response> {
  const filePath = `${collection_path}/${url.pathname.substring(12)}`;

  const [fileExists, _fileInfo] = await getFileInfo(filePath);

  if (!fileExists) {
    return new Response(null, { status: 404 });
  }

  if (_fileInfo?.isDirectory) {
    return new Response("PATH IS A DIRECTORY", {
      status: 400,
    });
  }

  const fileContent = await Deno.readTextFile(filePath);
  const frontmatters = extractFrontMatter(fileContent);

  frontmatters["last_modified"] = _fileInfo.mtime ? Math.floor(_fileInfo.mtime.getTime() / 1000) : null;

  return new Response(JSON.stringify(frontmatters), {
    status: 200,
    headers: {
      "Content-Type": "application/json",
    },
  });
}

function canBeNumber(value: string): boolean {
  const number = Number(value);
  return !isNaN(number);
}

function extractFrontMatter(content: string) {
  const split = content.split("\n");
  let startRecording = false;
  const frontmatter_data: { [key: string]: string | boolean | number } = {};

  for (const line of split) {
    if (line.startsWith("---") && startRecording) {
      break;
    }

    if (line.startsWith("---")) {
      startRecording = true;
      continue;
    }

    if (startRecording) {
      const value = line.split(": ");
      if (value.length < 2) {
        continue;
      }

      let final_value: string | boolean | number | undefined = undefined;

      if (value[1] == "true" || value[1] == "false") {
        final_value = value[1] === "true";
      }

      if (canBeNumber(value[1])) {
        final_value = Number(value[1]);
      }

      if (final_value === undefined) {
        if (value[1].startsWith(`"`) && value[1].endsWith(`"`)) {
          // Remove the quotes
          final_value = value[1].slice(1, -1); // Removes the first and last character
        } else {
          final_value = value[1]; // Assigns the value without modification
        }
      }

      frontmatter_data[value[0]] = final_value;
    }
  }

  return frontmatter_data;
}
