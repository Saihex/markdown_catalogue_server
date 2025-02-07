import { collection_path } from "../globals.ts";
import { getFileInfo } from "../utils/file_utils.ts";
import * as mrmime from "mrmime";

export async function files_api_handler(
  _req: Request,
  url: URL,
): Promise<Response> {
  const filePath = `${collection_path}/${url.pathname.substring(7)}`;

  const [fileExists, _fileInfo] = await getFileInfo(filePath);

  if (!fileExists) {
    return new Response(null, { status: 404 });
  }

  if (_fileInfo?.isDirectory) {
    return new Response("PATH IS A DIRECTORY\nConsider using /dirSearch API", {
      status: 400,
    });
  }

  const contentType = mrmime.lookup(filePath) || "application/octet-stream";
  const fileContent = await Deno.readTextFile(filePath);

  return new Response(fileContent, {
    headers: new Headers({
      "Content-Type": contentType,
      "Cache-Control": "public, max-age=7200, must-revalidate"
    }),
  });
}
