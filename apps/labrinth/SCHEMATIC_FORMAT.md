# ShroudEdit `.schematic` Format

ShroudEdit adapts the established Schematic concept for the Enshrouded World Editor. A `.schematic` file is a ZIP archive containing a manifest, preview image, and opaque World Editor payload. Schematics remain a dedicated ShroudEdit project type alongside Mods and Modpacks.

## Archive layout

```text
example.schematic
├── manifest.json
├── preview.webp
└── data/
    └── structure.bin
```

`manifest.json` uses UTF-8 JSON:

```json
{
	"formatVersion": 1,
	"name": "Mountain workshop",
	"version": "1.0.0",
	"gameVersion": "0.9.1.2",
	"loaders": ["eml"],
	"worldEditorVersion": "0.4.0",
	"dimensions": {
		"width": 48,
		"height": 26,
		"depth": 32
	},
	"payload": {
		"path": "data/structure.bin",
		"sha512": "128 lowercase hexadecimal characters"
	},
	"preview": {
		"path": "preview.webp"
	},
	"installationInstructions": "Import the file from the World Editor schematic browser.",
	"dependencies": [
		{
			"projectId": "dependency-project-id",
			"versionId": "dependency-version-id",
			"optional": false
		}
	]
}
```

## Validation

- The filename ends in `.schematic` and the file is a valid ZIP archive.
- `formatVersion` is currently `1`.
- `version`, `gameVersion`, `worldEditorVersion`, all dimensions, `payload`, and `preview` are required.
- `loaders` contains Shroudtopia, Shroudforge, or both.
- Width, height, and depth are integers from `1` through `4096`.
- `payload.sha512` matches the uncompressed payload referenced by `payload.path`.
- Every referenced file exists in the archive.
- Each optional dependency identifies a ShroudEdit project, a version, or both.

Labrinth additionally calculates SHA-1 and SHA-512 checksums for the complete `.schematic` archive. `.mrpack` and its existing Modpack processing remain unchanged.
