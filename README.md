<p align="center">ccrs | conventional commits and semantic bumps</p>

<br>

Notice

&nbsp;&nbsp;&nbsp;&nbsp;This tool is one I've created for myself. I'm open to suggestions and improvements, so please do open an issue if you have anything to share. However, I'm quite a minimalist, so expect this project to follow suit.

<br>

Installation

```bash
brew tap tofuuudon/ccrs
brew install ccrs
```

<br>

Commit

&nbsp;&nbsp;&nbsp;&nbsp;You can specify which type of commit you'd like to create, e.g. `ft`, `fx`, and `rf` — which are `feat`, `fix`, and `refactor`, respectively. There are more types, please refer to the table below.

|    longhand      |   shorthands  |
|----------|-----|
| feat     | ft,fe  |
| fix      | fx,fi  |
| docs     | dc,do  |
| style    | stl,st |
| perf     | pf,pe  |
| test     | ts,te  |
| build    | bd,bu  |
| ci       | ci  |
| chore    | chr,ch |
| refactor | rf,re  |

https://github.com/user-attachments/assets/b286979f-4b41-412c-ad14-87ba8926269c

<br>

Bump

&nbsp;&nbsp;&nbsp;&nbsp;You must have an existing initial git tag for `ccrs bump` to work. It takes the existing semantic version, and bumps it by a major if it has a breaking change, a minor for `feat`, and a patch for `fix`. It will reuse any prefixes like `v` from the previous tag.

https://github.com/user-attachments/assets/9843697b-56fa-4178-8abd-07c03a9f87ca

