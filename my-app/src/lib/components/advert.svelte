<script lang="ts">
  export let title;
  export let description;
  export let date;
  export let location;
  export let price;
  export let oldPrice;
</script>

<div class="advert">
  <div class="photos">
    <img src="" alt="" />
    <div class="additional-photos">
      <img src="" alt="" />
      <img src="" alt="" />
    </div>
  </div>
  <div class="info">
    <div class="title">{title}</div>
    <div class="description">{description}</div>
    <div class="date">{date}</div>
    <div class="location">{location}</div>
    <div class="price">{price}</div>
  </div>
</div>

<style lang="scss">
</style>
