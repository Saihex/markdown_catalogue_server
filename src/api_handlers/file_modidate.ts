import { collection_path } from "../globals.ts";
import { getFileInfo } from "../utils/file_utils.ts";
import * as mrmime from "mrmime";

export async function file_modidate_api_handler(
  _req: Request,
  url: URL,
): Promise<Response> {
  const filePath = `${collection_path}/${url.pathname.substring(13)}`;

  const [fileExists, _fileInfo] = await getFileInfo(filePath);

  if (!fileExists || !_fileInfo) {
    return new Response(null, { status: 404 });
  }

  if (_fileInfo.isDirectory) {
    return new Response("PATH IS A DIRECTORY\nConsider using /dirSearch API", {
      status: 400,
    });
  }

  const unixTimestamp = _fileInfo.mtime ? Math.floor(_fileInfo.mtime.getTime() / 1000) : null;

  return new Response(JSON.stringify(
    {
        unix_modification_date: unixTimestamp,
    }
  ), {
    headers: new Headers({
      "content-type": "application/json",
    }),
  });
}
