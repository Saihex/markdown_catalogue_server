# Saihex Studios' Markdown Catalogue Server
This software is used by [Saihex Studios](www.saihex.com) for the wiki website in order to catalogue the markdown files.

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

### List mode JSON body
List mode will return a JSON array of JSON dictionaries structured like this
```json
[
    {
        "title": "",
        "description": "",
        "image": "",
        "dynamic_path": ""
    }
]
```

The first 3 will be set to what it reads from the markdown front-matter. `dynamic_path` will be set to path relative to the current directory which is like this:

`/collection/something.md` -> `something`

removal of the `.md` is due to how our wiki website dynamic routes works. **List mode will only list 50 markdown files at once sorted by similarity first then alphabet.**