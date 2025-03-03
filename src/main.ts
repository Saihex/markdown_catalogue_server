import { files_api_handler } from "./api_handlers/files_handler.ts";
import { dirSearch_api_handler } from "./api_handlers/dirSearch_handler.ts";
import { sitemap_api_handler } from "./api_handlers/sitemap.ts";
import { frontmatter_api_handler } from "./api_handlers/frontmatter.ts";
import { file_modidate_api_handler } from "./api_handlers/file_modidate.ts";
import { collection_path } from "./globals.ts";

const VERSION = "v0.0.2-g";
type ApiHandler = (request: Request, url: URL) => Promise<Response>;

const handler_apis: Record<string, ApiHandler> = {
  files: files_api_handler,
  dirSearch: dirSearch_api_handler,
  sitemap: sitemap_api_handler,
  frontmatter: frontmatter_api_handler,
  fileModiDate: file_modidate_api_handler,
};

function getAPIType(path: string): string | null {
  const url_segments = path.split("/");

  if (url_segments.length > 0) {
    return url_segments[1];
  }

  return null;
}

async function main_handler(req: Request) {
  if (req.url.includes("..")) {
    return new Response(null, {
      status: 403,
      headers: {
        "MCSVersion": VERSION
      }
    });
  }

  const url = new URL(req.url, `http://${req.headers.get("host")}`);
  const api_type = getAPIType(url.pathname);

  if (!api_type) {
    return new Response("API Identifier not set.", {
      status: 400,
      headers: {
        "MCSVersion": VERSION
      }
    });
  }

  // the API handler execution.
  {
    if (api_type == "heartbeat") {
      return new Response("OK", {
        status: 200,

        headers: {
          "Content-Type": "text/plain",
          "Cache-Control": "no-cache",
          "MCSVersion": VERSION
        },
      });
    }

    const handler = handler_apis[api_type];

    if (handler) {
      const response = await handler(req, url);

      response.headers.set(
        "MCSVersion",
        VERSION,
      );

      return response;
    }
  }

  // no API type found;
  return new Response(`Invalid API Identifier.\nAPI Exists? ${api_type}`, {
    status: 400,
  });
}

console.log(collection_path);
Deno.serve({ port: 8080 }, main_handler);
