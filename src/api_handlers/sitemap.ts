import { collection_path } from "../globals.ts";

const exclusions: string[] = [
  ".gitattributes",
  ".vscode",
  "LICENSE",
  "readme.md",
  ".git",
];

export async function sitemap_api_handler(
  _req: Request,
  _url: URL,
): Promise<Response> {
  // searching logic
  {
    const dirRead = Deno.readDir(collection_path);
    const results: string[] = [];

    for await (const dirEntry of dirRead) {
      if (!exclusions.includes(dirEntry.name)) {
        results.push(`https://wiki.saihex.com/wiki/${dirEntry.name}`);
      }
    }

    const lastmod = new Date().toISOString().split("T")[0];

    const sitemap_text = generateSitemap(results, lastmod);

    return new Response(sitemap_text, {
      status: 200,
      headers: {
        "Content-Type": "application/xml; charset=utf-8",
      },
    });
  }
}

function generateSitemap(urls: string[], lastmod: string): string {
  const sitemapHeader = '<?xml version="1.0" encoding="UTF-8"?>\n' +
    '<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n';
  const sitemapFooter = "</sitemapindex>";

  // Create XML for each sitemap entry
  const sitemapEntries = urls
    .map((url) => {
      return `  <sitemap>\n    <loc>${url}</loc>\n    <lastmod>${lastmod}</lastmod>\n  </sitemap>`;
    })
    .join("\n");

  return sitemapHeader + sitemapEntries + "\n" + sitemapFooter;
}
