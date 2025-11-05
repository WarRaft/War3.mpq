import path from 'path'
import fs from 'fs'
import {rmDir} from './rmdir.mjs'

const base = path.join('..', 'extract')
const dest = path.join('..', 'lowercase')

rmDir(dest)

const forDir = parent => {
    if (!fs.existsSync(parent)) {
        console.error(`Not Exists parent: ${parent}`)
        return
    }

    for (const item of fs.readdirSync(parent)) {
        const current = path.join(parent, item)
        const stat = fs.statSync(current)
        if (stat.isDirectory()) {
            forDir(current)
            continue
        }
        const newdest = path.join(dest, path.relative(base, current).toLowerCase())
        const dirname = path.dirname(newdest)
        if (!fs.existsSync(dirname)) fs.mkdirSync(dirname, {recursive: true, mode: '0777'})

        fs.copyFileSync(current, newdest)
    }
}

forDir(path.join(base, 'Abilities'))
forDir(path.join(base, 'Buildings'))
forDir(path.join(base, 'Doodads'))
forDir(path.join(base, 'Environment'))
forDir(path.join(base, 'Objects'))
forDir(path.join(base, 'PathTextures'))
forDir(path.join(base, 'ReplaceableTextures'))
forDir(path.join(base, 'SharedModels'))
forDir(path.join(base, 'Textures'))
forDir(path.join(base, 'Units'))
forDir(path.join(base, 'UI'))
