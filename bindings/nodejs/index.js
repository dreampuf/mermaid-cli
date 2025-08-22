const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd 2>/dev/null', { encoding: 'utf8' })
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'mermaid-it.win32-x64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./mermaid-it.win32-x64-msvc.node')
          } else {
            nativeBinding = require('mermaid-it-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'mermaid-it.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./mermaid-it.darwin-x64.node')
          } else {
            nativeBinding = require('mermaid-it-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'mermaid-it.darwin-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./mermaid-it.darwin-arm64.node')
          } else {
            nativeBinding = require('mermaid-it-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(join(__dirname, 'mermaid-it.linux-x64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./mermaid-it.linux-x64-musl.node')
            } else {
              nativeBinding = require('mermaid-it-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'mermaid-it.linux-x64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./mermaid-it.linux-x64-gnu.node')
            } else {
              nativeBinding = require('mermaid-it-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(join(__dirname, 'mermaid-it.linux-arm64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./mermaid-it.linux-arm64-musl.node')
            } else {
              nativeBinding = require('mermaid-it-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'mermaid-it.linux-arm64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./mermaid-it.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('mermaid-it-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { NodeMermaidRenderer } = nativeBinding

module.exports.NodeMermaidRenderer = NodeMermaidRenderer
module.exports.MermaidRenderer = NodeMermaidRenderer // Alias for convenience