import { collection_path } from "../globals.ts";

const exclusions: string[] = [
  ".gitattributes",
  ".vscode",
  "LICENSE",
  "readme.md",
  ".git",
];

export async function sitemap_api_handler(
  req: Request,
  _url: URL,
): Promise<Response> {
  // searching logic
  {
    const url = new URL(req.url);
    const directory = url.searchParams.get("wiki") || collection_path;
    const layer0 = url.searchParams.get("wiki") == null;

    let searchDir;

    if (layer0 == true) {
      searchDir = collection_path;
    } else {
      searchDir = collection_path + "/" + directory;
    }

    const dirRead = Deno.readDir(searchDir);
    const results: string[] = [];

    for await (const dirEntry of dirRead) {
      if (!exclusions.includes(dirEntry.name)) {
        if (layer0) {
          results.push(
            `https://wiki.saihex.com/api/msc/sitemap?wiki=${dirEntry.name}/category`,
          );
        } else {
          results.push(
            `https://wiki.saihex.com/wiki/${directory}/${dirEntry.name}`,
          );
        }
      }
    }

    const lastmod = new Date().toISOString().split("T")[0];

    let sitemap_text;
    
    if (layer0) {
      sitemap_text = generateSitemap_layer0(results, lastmod);
    } else {
      sitemap_text = generateSitemap_layer1(results, lastmod);
    }

    return new Response(sitemap_text, {
      status: 200,
      headers: {
        "Content-Type": "application/xml",
        "Cache-Control": "no-cache",
      },
    });
  }
}

function escapeXml(str: string): string {
  return str.replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;")
            .replace(/"/g, "&quot;")
            .replace(/'/g, "&apos;");
}

// wiki listing
function generateSitemap_layer0(urls: string[], lastmod: string): string {
  const sitemapHeader = '<?xml version="1.0" encoding="UTF-8"?>\n' +
    '<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n';
  const sitemapFooter = "</sitemapindex>";

  const sitemapEntries = urls
    .map((url) => {
      return `  <sitemap>\n    <loc>${escapeXml(url)}</loc>\n    <lastmod>${lastmod}</lastmod>\n  </sitemap>`;
    })
    .join("\n");

  return sitemapHeader + sitemapEntries + "\n" + sitemapFooter;
}

// wiki's category listing
function generateSitemap_layer1(urls: string[], lastmod: string): string {
  const sitemapHeader = '<?xml version="1.0" encoding="UTF-8"?>\n' +
    '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n';
  const sitemapFooter = "</urlset>";

  const sitemapEntries = urls
    .map((url) => {
      return `  <url>\n    <loc>${escapeXml(url)}</loc>\n    <lastmod>${lastmod}</lastmod>\n    <changefreq>weekly</changefreq>\n  </url>`;
    })
    .join("\n");

  return sitemapHeader + sitemapEntries + "\n" + sitemapFooter;
}

