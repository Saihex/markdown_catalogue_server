import { collection_path } from "../globals.ts";
import { getFileInfo } from "../utils/file_utils.ts";
import { TextLineStream } from "jsr:@std/streams@0.223.0/text-line-stream";
import { walk } from "jsr:@std/fs/walk";

export async function frontmatter_api_handler(
  _req: Request,
  url: URL,
): Promise<Response> {
  const filePath = `${collection_path}/${url.pathname.substring(12)}`;

  const [fileExists, _fileInfo] = await getFileInfo(filePath);

  // safety logic
  {
    //
    if (!fileExists) {
      return new Response(null, { status: 404 });
    }

    //
    if (!_fileInfo) {
      return new Response("Couldn't get file info!", {
        status: 500,
      });
    }

    //
    if (_fileInfo?.isDirectory) {
      return new Response("PATH IS A DIRECTORY", {
        status: 400,
      });
    }
  }

  let frontmatters;

  // extraction logic
  try {
    frontmatters = await extractFrontMatter(filePath);
  } catch (e) {
    return new Response(`Failed to extract frontmatters\n${e}`, {
      status: 500,
    });
  }

  frontmatters["last_modified"] = _fileInfo.mtime
    ? Math.floor(_fileInfo.mtime.getTime() / 1000)
    : 0;

    if (url.searchParams.get("markdown_count") === "true") {
      const pathSegments = filePath.split("/");
      pathSegments.pop();
  
      const directoryPath = pathSegments.join("/");
      frontmatters["page_count"] = await countMarkdownFiles(directoryPath);
  }

  return new Response(JSON.stringify(frontmatters), {
    status: 200,
    headers: {
      "Content-Type": "application/json",
      "Cache-Control": "public, max-age=3600, must-revalidate"
    },
  });
}

function canBeNumber(value: string): boolean {
  const number = Number(value);
  return !isNaN(number);
}

async function extractFrontMatter(file_path: string) {
  const file = await Deno.open(file_path, { read: true });

  const reader = file.readable
    .pipeThrough(new TextDecoderStream())
    .pipeThrough(new TextLineStream());

  let startRecording = false;
  const frontmatter_data: { [key: string]: string | boolean | number } = {};

  for await (const line of reader) {
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

async function countMarkdownFiles(dir: string): Promise<number> {
  let count = 0;

  // Walk through the directory recursively
  for await (const entry of walk(dir)) {
    if (entry.isFile && entry.name.endsWith(".md")) {
      count++;
    }
  }

  return count;
}
