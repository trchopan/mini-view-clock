<template>
  <div class="about-container">
    <div class="about">
      <p>Just a simple clock</p>
      <p class="small">Made by <a href="https://chop.dev">Chop Tr</a></p>
    </div>
  </div>
</template>

<style>
  .about-container {
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
  }
  .about > .small {
    font-size: 0.7rem;
    color: var(--shade);
  }
  a {
    color: var(--shade);
    font-weight: bold;
  }
</style>
