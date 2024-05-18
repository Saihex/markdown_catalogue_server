**Version: 0.0.2-c**

Note: Letters after the version numbers are their sub-evolutions when non-major changes were added. Version number will increase after the letter hits `z`.
If we made a push and realized there is a bug or mistake in the code that is lethal we will delete the Docker tag and push the fixes under the same version tag.

# Saihex Studios' Markdown Catalogue Server
<img align="right" width="128" src="https://img.saihex.com/software_logos/markdown_catalogue_server.svg">

This software is used by [Saihex Studios](www.saihex.com) for the wiki website in order to catalogue the markdown files.
Meant to be used with [Saihex Studios' Nuxt Markdown Wiki Website](https://github.com/Saihex/nuxt-static-markdown-wiki-website)

- Expose port `8080`
- Attach `/collection` volume to the directory that contains all the images and files you want to expose.

## File structure
Any path given will be used as the path from the collection directory. Example:
`localhost:8080/FranchiseName/Stories/the.md` -> `/collection/FranchiseName/Stories/the.md`

If given path is a directory; the handler will enter list mode, the requester can put a parameter `dir_search` to string value of the markdown file to search. Be warned that this won't search by the front-matter `title` within the markdown files so it is best to keep the two almost identical or better exactly the same.

If given path is a file; the handler will read it and pass it as body, basically acting as a generic file server. **This doesn't limit to just markdown files unlike list mode.**

## List mode
### ***WARNING***
front-matter of every markdown files must have `title`, `description` and `image` string values. Without this, the server will still list it but the string value for them will be empty. No additional front-matters will be included. Files with `index` as `dynamic_path` will be removed from the list.

### Front-matter parsing
The software will read the front-matter and make a object of it. Default value of the type is used if the front-matter value isn't found.

For those with sensitive characters please use **quotation mark** in between the String value. The software will automatically remove the quotes from the String value on parsing

**Front-matters that are not in the expected list will not be included into the parsed data.**

### Expected franchise markdown front-matters
```rust
franchise_proper_name: String, // Proper name of the franchise
title: String, // Title of the page, normally "Home"
description: String, // Description of the franchise
ico_image: String, // The tab icon
wiki_head_image: String, // The image that appears on top-left of the wiki page.
default_embed_image: String, // Default embed image which is used for main page.
image: String, // Square logo image
saihex_creation: true // Optional. Used to determine whether it is Saihex's creation. Default to false if none found.
```

### Expected Front-matters within markdown files
```rust
title: String,
description: String,
image: String, // The image shown at the search results and page embed
spoiler: bool // optional, if none the software will assume it as false.
```

### List mode JSON body
List mode will return a JSON array of JSON dictionaries structured like this
(Defaults)
```json
[
    {
        "title": "",
        "description": "",
        "image": "",
        "dynamic_path": "",
        "spoiler": false,
    }
]
```

The first 3 will be set to what it reads from the markdown front-matter. `dynamic_path` will be set to path relative to the current directory which is like this:

`/collection/something.md` -> `something`

removal of the `.md` is due to how our wiki website dynamic routes works. **List mode will only list 50 markdown files at once sorted by similarity first then alphabet.**

Used dependencies
```
async-stream = "0.3.5" --  MIT
actix-web = "4" --  MIT OR Apache-2.0 
actix-files = "0.6" --  MIT OR Apache-2.0 
rust_search = "2.0.0" --  MIT
serde_json = "1.0" --  MIT OR Apache-2.0 
serde = { version = "1.0", features = ["derive"] } --  MIT OR Apache-2.0
serde-xml-rs = "0.6.0" -- MIT
```

**Docker Image**
```
saihex/markdown_catalogue_server:v0.0.2-c
```