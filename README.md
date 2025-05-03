# mcp-op-return

An MCP server for utilizing opreturnbot.com

## Setup

1. Install

```
git clone https://github.com/benthecarman/mcp-op-return.git
cd mcp-op-return
cargo install --path .
```

2. Add to mcp config

```
{
    "mcpServers": {
       "mcp-op-return": {
          "command": "mcp-op-return",
          "args": [
             "--mcp"
          ]
       }
    }
 }
```
