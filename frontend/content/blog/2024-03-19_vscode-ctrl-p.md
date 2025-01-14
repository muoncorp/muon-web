+++
title = "Visual Studio Code의 터미널에서 Ctrl-P 사용하기"

[taxonomies]
categories = ["tips"]
tags = ["vscode", "shell"]
authors = ["ceo"]
+++

bash나 zsh 같은 shell을 사용할 때 자주 사용하게 되는 단축키로 Ctrl-P 키가 있다.
이 단축키의 기능은 이전 실행 커맨드를 불러와주는 것으로 Up 화살표 키와 동일한 기능이다.

<!-- more -->

왜인지는 모르겠지만 방향키보다는 더 자주 사용하게 된다.
과거 이맥스(emacs)의 키바인딩에서 가져온 잔재라고도 볼 수 있는데,
Ctrl-A, Ctrl-E 역시 자주 쓰고 있다.

최근에 개발용 에디터로 vscode를 많이 쓰게 되면서 자주 쓰게 되는 단축키 역시
Ctrl-P이다. 파일을 검색해서 이동하는 기능인다.

문제는 vscode의 터미널로 shell을 이용할 때 Ctrl-P를 누르면 vscode의 단축키가
적용된다. 이럴 때 터미널에서는 Ctrl-P가 shell에서 동작하도록 하는 방법이 있다.

vscode의 명령 팔레트(Ctrl-Shift-P)에서 Open User Settings (JSON)을 찾아서
설정 파일을 열어준다. 그리고 아래와 같은 내용을 추가하여 준다.

```json
    "terminal.integrated.commandsToSkipShell": [
        "-workbench.action.quickOpen"
    ]
```