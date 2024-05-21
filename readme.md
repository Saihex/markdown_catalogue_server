**Version: 0.0.2-d**

Note: Letters after the version numbers are their sub-evolutions when non-major changes were added. Version number will increase after the letter hits `z`. If we found lethal issues after a push we will delete the last image build from docker hub and push fixes under the same version. Version won't increase if changes are less than `three` unless huge change such as core rework.

# Saihex Studios' Markdown Catalogue Server
<img align="right" width="128" src="https://img.saihex.com/software_logos/markdown_catalogue_server.svg">

This software is used by [Saihex Studios](www.saihex.com) for the wiki website in order to catalogue the markdown files.
Meant to be used with [Saihex Studios' Nuxt Markdown Wiki Website](https://github.com/Saihex/nuxt-static-markdown-wiki-website)

**WARNING: This software is designed to be used within a Docker container**

- Expose port `8080`
- Attach `/collection` volume to the directory that contains all the images and files you want to expose.

## Docker setup with Saihex Studios' Nuxt Markdown Wiki Website
When being used with [Saihex Studios' Nuxt Markdown Wiki Website](https://github.com/Saihex/nuxt-static-markdown-wiki-website) - This software's container must be named `markdown_cat_server` and share the same Docker network.

It is not necessary to expose the port of this software's container in this setup.

---

## File Structure
A directory `/collection` containing all the wiki must be present for this software to work and follow this structure (franchises are individual wikis)
```
/collection
    /franchise1
        /category1
            /page1
            /page2

        /category2
            /page1
            /page2
    
    /franchise2
        /category1
            /page1
            /page2
            
        /category2
            /page1
            /page2
```

## Fetch markdown file
To get the markdown is simply by providing absolute path to the file starting from the collection directory.

**Example:** `http://localhost:8080/The4Tris/logic/Brain_Crystal.md`

## Search APIs
### Wiki searching
To list and search all available wikis - the url must be anything but root path and includes a query parameter `root_dir_search` with value of `true`. Query parameter `search_input` is used as well, the search input.

The search input is typo tolerance. The software will try to read the wiki main page `franchise_proper_name` [front-matter](#expected-markdown-front-matters) and if it can't it will use the franchise directory name. 

**Examples:**

- Anything but root path: `http://localhost:8080/a?root_dir_search=true`
- with search input: `http://localhost:8080/a?search_input=The4Tris&root_dir_search=true`

### Category Searching
To list and search all available categories within a wiki - the url must points to the wiki alongside query parameter `category_search` with value of `true`. Query parameter `search_input` is used as well to serve same purpose and logic flow as it does in [Wiki searching](#wiki-searching).

However, the search input in this case will instead look for `title` [front-matter](#expected-markdown-front-matters) rather than `franchise_proper_name`. 

**Examples:**
- without search input: `http://localhost:8080/The4Tris?category_search=true`
- with search input: `http://localhost:8080/The4Tris?category_search=true&search_input=logic`

### Category Contents Searching
To list and search all available pages within a wiki's category - the url must points to that wiki category without requiring any query parameter to active the searching feature. Query parameter `search_input` is used as well to serve same purpose and logic flow as it does in [Category Searching](#category-searching).

**Examples:**
- without search input: `http://localhost:8080/The4Tris/logic`
- with search input: `http://localhost:8080/The4Tris/logic?search_input=Brain`

## Expected markdown front-matters
**For franchises. Example taken from [The 4Tris wiki homepage](https://wiki.saihex.com/wiki/The4Tris) markdown.**
```yaml
franchise_proper_name: The 4Tris
title: Home
description: Four Brain Crystal Robots doing missions while attached to a reality bending simulation to save the world.
ico_image: https://img.saihex.com/wiki_exclusive/The4Tris/The4Tris.ico
wiki_head_image: https://img.saihex.com/wiki_exclusive/The4Tris/The4Tris_Cover_Text.svg
default_embed_image: https://img.saihex.com/webp?src=wiki_exclusive/The4Tris/4TrisCover.png
image: https://img.saihex.com/wiki_exclusive/The4Tris/The4Tris.svg
saihex_creation: true
```

**For categories and pages. Example taken from [The 4Tris wiki Sairo](https://wiki.saihex.com/wiki/The4Tris/category/Characters/Sairo) character page.**
```yaml
title: Sairo
description: Sairo, a powerful little Brain Crystal that is also half Reality Bender Crystal. She is one of the four main characters.
image: https://img.saihex.com/webp?src=wiki_exclusive/The4Tris/page_icon/characters/sairo/sairo.png
spoiler: false
```

## JSON Bodies
The list of dynamically created data (both bodies):
- `last_modified`
- `page_count`
- `dynamic_path`

**Wiki search**
```json
[
    {
        "default_embed_image": "https://img.saihex.com/webp?src=wiki_exclusive/The4Tris/4TrisCover.png",
        "description": "Four Brain Crystal Robots doing missions while attached to a reality bending simulation to save the world.",
        "dynamic_path": "The4Tris",
        "franchise_proper_name": "The 4Tris",
        "ico_image": "https://img.saihex.com/wiki_exclusive/The4Tris/The4Tris.ico",
        "image": "https://img.saihex.com/wiki_exclusive/The4Tris/The4Tris.svg",
        "last_modified": 1716099640,
        "page_count": 0,
        "saihex_creation": true,
        "title": "Home",
        "wiki_head_image": "https://img.saihex.com/wiki_exclusive/The4Tris/The4Tris_Cover_Text.svg"
    }
]
```
**Category and page search**
```json
[
    {
        "description": "All the characters in The 4Tris universe (excluding background characters)",
        "dynamic_path": "Characters",
        "image": "https://img.saihex.com/webp?src=wiki_exclusive/The4Tris/category_icons/characters.png",
        "last_modified": 1716099640,
        "spoiler": false,
        "title": "Characters"
    }
]
```

## Sitemaps
Sitemap is used by front-end to provide web crawlers path of URLs to properly index the wiki website.

### Sitemap Structure
`/sitemap_xml` by itself provides locations to get the sitemaps that provides each wiki's pages and categories URLs. (Ex: `/sitemap_xml/The4Tris`)

`/sitemap` provides location to wiki home pages.

---


## Additional data
**Used dependencies (TOTAL `MIT*`)**
```
async-stream = "0.3.5" --  MIT
actix-web = "4" --  MIT OR Apache-2.0 
actix-files = "0.6" --  MIT OR Apache-2.0 
serde_json = "1.0" --  MIT OR Apache-2.0 
serde = { version = "1.0", features = ["derive"] } --  MIT OR Apache-2.0
chrono = "0.4" -- MIT or Apache-2.0
strsim = "0.11.1" -- MIT
rayon = "1.5.3" -- MIT or Apache-2.0
```

**Docker Image**
```
saihex/markdown_catalogue_server:v0.0.2-d
```
