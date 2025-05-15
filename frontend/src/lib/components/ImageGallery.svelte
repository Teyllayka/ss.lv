<script lang="ts">
  const { images } = $props<{ images: string[] }>();

  let currentImageIndex = $state(0);
  let isHovering = $state(false);

  function showImage(index: number) {
    currentImageIndex = index;
  }
</script>

<div class="thumbnail relative h-[200px]">
  {#each images as image, index}
    <div
      class="image-container absolute inset-0 transition-opacity duration-200"
      style:opacity={(!isHovering && index === 0) ||
      (isHovering && index === currentImageIndex)
        ? "1"
        : "0"}
      style:z-index={(!isHovering && index === 0) ||
      (isHovering && index === currentImageIndex)
        ? "10"
        : "0"}
    >
      <img
        class="h-full w-full object-contain"
        src={image}
        alt={`Image ${index + 1}`}
      />
    </div>
  {/each}

  {#if images.length > 5}
    <div
      class="last-pager absolute inset-0 bg-black/50 text-white grid place-items-center transition-opacity duration-200"
      style:opacity={isHovering && currentImageIndex === 4 ? "1" : "0"}
      style:pointer-events={isHovering && currentImageIndex === 4
        ? "auto"
        : "none"}
    >
      +{images.length - 5} images
    </div>
  {/if}

  <div
    class="h-full w-full absolute inset-0 flex"
    on:mouseenter={() => (isHovering = true)}
    on:mouseleave={() => (isHovering = false)}
  >
    {#each images.slice(0, 5) as _, index}
      <div class="hover-area flex-1" on:mouseenter={() => showImage(index)}>
        <div
          class="indicator absolute left-[10px] right-[10px] h-[2px] bottom-[10px] transition-colors duration-200"
          style:background-color={currentImageIndex === index && isHovering
            ? "rgb(59, 130, 246)"
            : "gray"}
        ></div>
      </div>
    {/each}
  </div>
</div>

<style>
  .thumbnail {
    position: relative;
    height: 200px;
    overflow: hidden;
  }

  .image-container img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .hover-area {
    position: relative;
    z-index: 20;
  }
</style>
