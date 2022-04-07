local meta = "https://nhentai.net/api/gallery/"..URL:match("g/(%d+)")
meta = web.get(meta)
meta = json.decode(meta)

IMAGE_URL = "https://i.nhentai.net/galleries/"..meta["media_id"].."/"
EXTS = { ["j"] = "jpg", ["p"] = "png", ["g"] = "gif" }

for i, tag in ipairs(meta["tags"]) do
    if tag["type"] == "tag" then tag["type"] = "general" end
    tmdl.push_tag(tag["type"], tag["name"])
end

for i, page in ipairs(meta["images"]["pages"]) do
    print(IMAGE_URL..i.."."..EXTS[page["t"]])
end

tmdl.push_tag("title", meta["title"]["pretty"])