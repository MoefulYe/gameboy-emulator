<template>
  <div id="loading">
    <div class="loading-text">
      <span>l</span>
      <span>o</span>
      <span>a</span>
      <span>d</span>
      <span>i</span>
      <span>n</span>
      <span>g</span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
$t: 2s;
$n: 7;
#loading {
  position: fixed;
  z-index: 9999;
  inset-block: 0;
  inset-inline: 0;
  background-color: #8fbcbb;
  display: flex;
  justify-content: center;
  align-items: center;
}

.loading-text {
  font-size: min(#{100vw * 0.05}, 3rem);
  color: #fff;
  text-transform: uppercase;
  position: relative;
  bottom: 5%;
  &::after {
    content: '';
    display: block;
    height: 0.1em;
    inset-block-end: 0;
    inset-inline: 0;
    background-color: #fff;
    position: absolute;
    animation: line $t ease-in-out infinite;
  }

  span {
    display: inline-block;
    padding: 0.6em;
    animation-name: char;
    animation-duration: $t;
    animation-timing-function: ease-in-out;
    animation-iteration-count: infinite;
    animation-fill-mode: backwards;
    @for $i from 1 through $n {
      &:nth-child(#{$i}) {
        animation-delay: calc($i * $t * 0.25 / $n);
      }
    }
  }
}

@keyframes line {
  from {
    inset-inline-end: 100%;
    inset-inline-start: 0;
    opacity: 0.1;
  }
  50% {
    inset-inline-end: 0;
    inset-inline-start: 0;
    opacity: 1;
  }
  to {
    inset-inline-end: 0;
    inset-inline-start: 100%;
    opacity: 0.1;
  }
}

@keyframes char {
  from {
    transform: translateY(1.8em);
    opacity: 0;
  }

  25%,
  50% {
    transform: none;
    opacity: 1;
  }

  75%,
  to {
    transform: translateY(-1.8em);
    opacity: 0;
  }
}
</style>
