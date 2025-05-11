<script lang="ts">
  export let images: string[] = [];
</script>

<div class="thumbnail relative h-[200px]">
  {#each images as image, index}
    <div class="bg-white dark:bg-gray-800 li">
      <img class="h-full w-full" src={image} alt={`Image ${index + 1}`} />
    </div>
  {/each}

  {#if images.length > 5}
    <div class="last-pager">
      +{images.length - 5} images
    </div>
  {/if}

  <div class="h h-full w-full">
    {#each images.slice(0, 5) as _, index}
      <div class="hovers relative"></div>
    {/each}
  </div>
</div>

<style lang="scss">
  .thumbnail {
    position: relative;
    height: 200px;
    overflow: hidden;
  }

  .thumbnail .li {
    position: absolute;
    inset: 0;
    z-index: -1;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .thumbnail:not(:has(.hovers:hover)) .li:first-of-type {
    opacity: 1;
  }

  .thumbnail:has(.hovers:nth-of-type(1):hover) .li:nth-of-type(1) {
    opacity: 1;
  }
  .thumbnail:has(.hovers:nth-of-type(2):hover) .li:nth-of-type(2) {
    opacity: 1;
  }
  .thumbnail:has(.hovers:nth-of-type(3):hover) .li:nth-of-type(3) {
    opacity: 1;
  }
  .thumbnail:has(.hovers:nth-of-type(4):hover) .li:nth-of-type(4) {
    opacity: 1;
  }
  .thumbnail:has(.hovers:nth-of-type(5):hover) .li:nth-of-type(5) {
    opacity: 1;
  }

  .thumbnail .li:nth-of-type(n + 6) {
    display: none;
  }

  .thumbnail .li img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .thumbnail .h {
    display: flex;
    position: absolute;
    inset: 0;
    opacity: 0;
    transition: opacity 0.2s ease;
    pointer-events: none;
  }
  .thumbnail:hover .h {
    opacity: 1;
    pointer-events: auto;
  }

  .thumbnail .hovers {
    flex: 1 1 0;
    position: relative;
  }

  .thumbnail .hovers::after {
    content: "";
    position: absolute;
    bottom: 10px;
    left: 10px;
    right: 10px;
    height: 2px;
    background-color: gray;
    transition: background-color 0.2s ease;
  }

  .thumbnail .hovers:hover::after {
    background-color: blue;
  }

  .thumbnail .last-pager {
    display: grid;
    place-items: center;
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    color: white;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s ease;
  }

  .thumbnail:has(.hovers:nth-of-type(5):hover) .last-pager {
    opacity: 1;
    pointer-events: auto;
  }

  .thumbnail .hovers:nth-of-type(n + 6) {
    display: none;
  }
</style>
