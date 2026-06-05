document.addEventListener('DOMContentLoaded', () => {

  // Get all "navbar-burger" elements
  const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);

  // Add a click event on each of them
  $navbarBurgers.forEach(el => {
    el.addEventListener('click', () => {

      // Get the target from the "data-target" attribute
      const target = el.dataset.target;
      const $target = document.getElementById(target);

      // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
      el.classList.toggle('is-active');
      $target.classList.toggle('is-active');

    });
  });

  // [한글 주석] 스크롤 감지하여 navbar에 .is-scrolled 클래스 토글 (루프 외부로 격리)
  const $navbar = document.querySelector('.navbar.is-fixed-top');
  if ($navbar) {
    const handleScroll = () => {
      if (window.scrollY > 450) {
        $navbar.classList.add('is-scrolled');
      } else {
        $navbar.classList.remove('is-scrolled');
      }
    };

    // [한글 주석] 페이지 로드 시점의 스크롤 위치도 확인하여 초기 클래스 반영
    handleScroll();

    // [한글 주석] 스크롤 이벤트 리스너 등록
    window.addEventListener('scroll', handleScroll);
  }
});
