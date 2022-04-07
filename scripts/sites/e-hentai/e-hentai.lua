function fmatch(str, pattern)
    local match = str:match(pattern)
    local i, j = str:find(match)
    str = str:sub(j + 1)
    return match, str
end

function get_torrents(meta)
    local torrents = web.get("https://e-hentai.org/gallerytorrents.php?gid="..meta["gid"].."&t="..meta["token"])
    local table = {table.unpack(meta["torrents"])}
    for i, t in ipairs(table) do
        table[i]["seeds"], torrents = fmatch(torrents, "Seeds:</span> (%d+)")
        table[i]["peers"], torrents = fmatch(torrents, "Peers:</span> (%d+)")
        table[i]["downloads"], torrents = fmatch(torrents, "Downloads:</span> (%d+)")
        table[i]["url"], torrents = fmatch(torrents, "\"(https://[^\"]+%.torrent)\"")
    end
    return table
end

function get_meta(url)
    local gid, token = url:match("g/(%d+)/(%w+)/")
    post = { ["method"] = "gdata", ["gidlist"] = { { gid, token } }, ["namespace"] = 1 }
    local meta = web.post("https://api.e-hentai.org/api.php", "application/json", json.encode(post))
    meta = json.decode(meta)
    meta = meta["gmetadata"][1]
    meta["torrents"] = get_torrents(meta)
    return meta
end

local meta = get_meta(URL)
for i, t in ipairs(meta["tags"]) do
    tmdl.push_tag(t:match("([^:]+):([^:]+)"))
end

tmdl.push_tag("title", meta["title"])