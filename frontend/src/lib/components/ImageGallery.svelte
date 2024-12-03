<script lang="ts">
export let images: string[] = [];
</script>

<div class="thumbnail relative h-[200px]">
  {#each images as image, index}
    <div class="li">
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

    .li {
      position: absolute;
      inset: 0;
      z-index: -1;

      img {
        object-fit: cover;
      }
    }

    .h {
      display: flex;
      opacity: 0;

      &:has(.hovers:nth-of-type(2)) {
        &:hover {
          opacity: 1;
        }
      }

      .hovers {
        flex: 1 1;

        &::after {
          position: absolute;
          content: "";
          height: 2px;
          bottom: 10px;
          inset-inline: 10px;
          background-color: gray;
        }

        &:hover::after {
          background-color: blue; /* Active color when hovered */
        }
      }
    }

    .li:not(:first-child) {
      opacity: 0;
    }

    &:has(.hovers:nth-of-type(1):hover) .li:nth-of-type(1) {
      opacity: 1;
    }
    &:has(.hovers:nth-of-type(2):hover) .li:nth-of-type(2) {
      opacity: 1;
    }
    &:has(.hovers:nth-of-type(3):hover) .li:nth-of-type(3) {
      opacity: 1;
    }

    .last-pager {
      display: grid;
      place-items: center;
      color: white;
      position: absolute;
      inset: 0;
      background: rgba(0, 0, 0, 0.5);
      opacity: 0;
      pointer-events: none;
    }

    &:has(.hovers:nth-of-type(6)) {
      &:has(.hovers:nth-of-type(5):hover) .last-pager {
        opacity: 1;
      }
    }

    .hovers:nth-of-type(n + 6) {
      display: none;
    }

    .li:nth-of-type(n + 6) {
      display: none;
    }
  }
</style>
