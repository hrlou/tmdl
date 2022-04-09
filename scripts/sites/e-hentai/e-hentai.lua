function fmatch(str, pattern)
    local match = str:match(pattern)
    local i, j = str:find(match)
    str = str:sub(j + 1)
    return match, str
end

function get_meta(url)
    local base, gid, token = url:match("https?://([%w%p]+)/g/(%d+)/(%w+)/")
    post = { ["method"] = "gdata", ["gidlist"] = { { gid, token } }, ["namespace"] = 1 }
    local meta = request("POST", "https://"..base.."/api.php", json.encode(post), { "Content-Type", "application/json" })
    meta = json.decode(meta)
    meta = meta["gmetadata"][1]
    meta["base"] = base
    meta["torrents"] = get_torrents(meta)
    return meta
end

function get_torrents(meta)
    local torrents = request("GET", "https://"..meta["base"].."/gallerytorrents.php?gid="..meta["gid"].."&t="..meta["token"])
    local table = { table.unpack(meta["torrents"]) }
    for i, t in ipairs(table) do
        table[i]["seeds"], torrents = fmatch(torrents, "Seeds:</span> (%d+)")
        table[i]["peers"], torrents = fmatch(torrents, "Peers:</span> (%d+)")
        table[i]["downloads"], torrents = fmatch(torrents, "Downloads:</span> (%d+)")
        table[i]["url"], torrents = fmatch(torrents, "\"(https://[^\"]+%.torrent)\"")
    end
    return table
end

local meta = get_meta(metadata.source)
for i, t in ipairs(meta["tags"]) do
    metadata:push_tag(t:match("([^:]+):([^:]+)"))
end

metadata.title = meta["title"]

-- print(json.encode(meta["torrents"]))
-- for i, t in ipairs(meta["torrents"]) do
--     print(t["url"])
-- end
-- tmdl.push_tag("title", meta["title"])