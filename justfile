default:
    just --list

update-lucide-static:
    npm ci
    cp node_modules/lucide-static/icon-nodes.json icon-nodes.json
