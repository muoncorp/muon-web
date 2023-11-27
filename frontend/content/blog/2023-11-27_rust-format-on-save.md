+++
title = "Visual Studio Code에서 Rust 소스 자동 포맷"

[taxonomies]
categories = ["rust"]
tags = ["rust", "vscode"]
authors = ["ceo"]
+++

vscode로 rust 코딩을 하다보면 '빠른 수정' 기능을 많이 쓰게 된다.
특히 use 문장이 지저분하게 되는 경우가 많은데, 이것을 일일이 정리하기는 상당히 귀찮은 일이다.

vscode의 'fromat document' 명령을 실행시켜서 정리를 할 수 있지만 이것도 귀찮기는 마찬가지이다.
최근 flutter를 사용하여 코딩하면서 편했던 것이 왠만한 것은 자동으로 포맷을 맞춰 준다는 것이었다.
파일을 저장하게 되면 자동으로 포맷이 되었다.
<!-- more -->

그래서 rust로 코딩할 때에도 비슷한 기능이 있지 않을까 찾아봤더니 역시 가능했다.
vscode로 rust를 코딩한다고 하면 거의 rust-analyzer를 사용하고 있을 것이다.
그러면 rust 프로젝트 폴더의 .vscode/settings.json 파일에 아래처럼 만들어주면 된다.

```toml
{
    "rust-analyzer.linkedProjects": [
        "./Cargo.toml"
    ],
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true
    }
}
```

이렇게 하면 파일을 저장하면 자동으로 포맷이 적용된다.
