+++
title = "Funtoo Linux 설치 문서 리뷰"

[taxonomies]
categories = ["linux"]
tags = ["funtoo"]
authors = ["ceo"]
+++

최근 [Funtoo Linux](https://funtoo.org)를 오랜만에 새로 설치해서 사용하게 되었다.
공식 사이트의 [설치 문서](https://www.funtoo.org/Install/Introduction)를 따라서 설치를 진행하였다.
설치하면서 느꼈던 설치 문서의 여러 페이지 중에서 보완하여 알면 좋은 팁들을 정리하였다.

<!-- more -->

### [Download LiveCD](https://www.funtoo.org/Install/Download_LiveCD)

리눅스를 새로 설치하려면 LiveCD(또는 LiveUSB)를 만들어야 한다.
예전에는 Funtoo Linux 전용의 LiveCD가 제공되지 않았던 것 같은데 지금은 Funtoo Linux 전용으로 LiveCD가 있다. 다른 배포판의 LiveCD들은 X-Window 환경의 GUI 인터페이스 설치를 지원하지만, Funtoo Linux는 텍스트 기반이다.
그렇지만 큰 문제는 아니고 오히려 Funtoo Linux와 어울리는 환경이라고 생각한다.

### [Prepare Disk](https://www.funtoo.org/Install/Prepare_Disk)

최신 PC에 설치를 하고 2TB를 초과하는 저장 장치에 설치하려면 반드시 UEFI/GPT 방식으로 설치를 진행해야 한다.
그렇지 않다고 해도 전통적인 MBR보다는 UEFI/GPT로 사용해 보기를 권한다.

### [Creating Filesystems](https://www.funtoo.org/Install/Prepare_Disk)

/boot 파티션을 제외한 다른 파티션들은 파일 시스템으로 XFS를 사용하기를 권한다.

### [Setting the Date](https://www.funtoo.org/Install/Setting_the_Date)

시간을 세팅하고 나서는 반드시 `hwclock --systohc` 실행시켜주길 권한다.
그렇지 않으면 설치 완료 후 부팅했을 때 시간이 틀어져 설치된 파일들이 미래 시간이 되어 버리면 emerge로 패키지 설치 시 굉장히 느려지는 문제가 발생할 수 있다.

### [Download and Extract Stage3](https://www.funtoo.org/Install/Download_and_Extract_Stage3)

stage3보다는 gnome tarball을 선택하는 게 시간을 줄일 수 있다.

### [Configuration Files](https://www.funtoo.org/Install/Configuration_Files)

/etc/fstab 파일은 반드시 수정해야 한다.
설치 문서에는 나오지 않지만 /dev/sda1, /dev/nvme0n1p1 같이 디바이스명을 직접 지정하지 말고 UUID를 지정해 주는 것을 강력히 권한다.
디바이스명을 직접 지정하는 경우 여러 개의 하드디스크나 SSD를 사용하는 경우 번호가 바뀌는 일이 발생할 수 있다. nvme 같은 경우는 커널 버전에 따라서도 부팅 시 바뀌어 버리는 일도 있었다. 이렇게 되면 부팅 자체가 실패하게 되어 다시 복구하려면 굉장히 귀찮아진다.

아래와 같이 해주면 된다.
```
UUID=9753-97D1                                  /boot           vfat            noauto,noatime      1 2
UUID=255c840a-3f3d-4b71-b782-246e1dceed1f       none            swap            sw                  0 0
UUID=ddca4533-c8bd-428f-aafa-7cae04871eec       /               xfs             noatime             0 1
UUID=c4443ec2-22ed-4964-8123-47c549152ad3       /home           xfs             noatime             0 2
```
UUID를 알 수 있는 방법은 blkid 명령을 이용하면 된다.

한국어 사용을 위해서는 make.conf를 아래와 같이 해주면 좋다.
```
L10N="en-US ko-KR ko"
LINGUAS="en_US ko_KR"
```

### 패키지 설치

필요한 패키지들은 최초 LiveCD 부팅했을 때 같이 설치하거나, 설치 문서대로
설치가 끝나고 재부팅 이후에 진행하면 된다.

한글 글꼴은 noto-cjk를 설치해 준다.
한글 입력을 위해서는 ibus와 ibus-hangul를 설치하면 된다.

Funtoo Linux는 커뮤니티가 매우 작기 때문에 debian 계열이나 gentoo처럼 많은 패키지들이 공식적으로 관리되고 있지는 않다. 따라서 필요한 패키지가 emerge로 설치할 수 없으면 다른 방법으로 설치해야 하는 경우가 많이 생긴다.

이 경우에 사용할 수 있는 것이 Flatpak이다.
Flatpak으로는 웬만한 애플리케이션은 바로 설치하여 사용할 수 있다.
Steam도 Flatpak을 이용해 설치할 수 있다. Windows만 지원하는 게임도 Proton을 활성화하면 대부분 문제없이 실행할 수 있다.

개발자라면 docker를 많이 이용하게 될 것이다.

### 맺음말

Funtoo Linux는 사용자가 적어 리눅스 초보자라면 도움받을 수 있는 문서도 적고, 사용하기 쉽지는 않은 배포판일 것이다.
그렇지만 리눅스에 익숙하다면 매우 안정적이고, 특히 systemd를 좋아하지 않는 사용자라면
사용을 고려해 볼 만한 배포판일 것이다.
