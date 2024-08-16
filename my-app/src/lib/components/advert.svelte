<script lang="ts">
  export let title;
  export let description;
  export let date;
  export let location;
  export let price;
  export let oldPrice;

  function formatDateTime(dateString: string) {
    const date = new Date(dateString);

    const day = String(date.getDate()).padStart(2, "0");
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");

    const formattedDateTime = `${day}.${month} ${hours}:${minutes}`;

    return formattedDateTime;
  }
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
    <div class="date">
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M5 22C4.45 22 3.97933 21.8043 3.588 21.413C3.19667 21.0217 3.00067 20.5507 3 20V6C3 5.45 3.196 4.97934 3.588 4.588C3.98 4.19667 4.45067 4.00067 5 4H6V3C6 2.71667 6.096 2.47934 6.288 2.288C6.48 2.09667 6.71733 2.00067 7 2C7.28267 1.99934 7.52033 2.09534 7.713 2.288C7.90567 2.48067 8.00133 2.718 8 3V4H16V3C16 2.71667 16.096 2.47934 16.288 2.288C16.48 2.09667 16.7173 2.00067 17 2C17.2827 1.99934 17.5203 2.09534 17.713 2.288C17.9057 2.48067 18.0013 2.718 18 3V4H19C19.55 4 20.021 4.196 20.413 4.588C20.805 4.98 21.0007 5.45067 21 6V20C21 20.55 20.8043 21.021 20.413 21.413C20.0217 21.805 19.5507 22.0007 19 22H5ZM5 20H19V10H5V20Z"
          fill="#B3B3B3"
        />
      </svg>
      <p>{formatDateTime(date)}</p>
    </div>
    <div class="location">
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M12 21.325C11.7667 21.325 11.5333 21.2833 11.3 21.2C11.0667 21.1167 10.8583 20.9917 10.675 20.825C9.59167 19.825 8.63333 18.85 7.8 17.9C6.96667 16.95 6.271 16.0293 5.713 15.138C5.155 14.2467 4.73 13.3883 4.438 12.563C4.146 11.7377 4 10.95 4 10.2C4 7.7 4.80433 5.70833 6.413 4.225C8.02167 2.74167 9.884 2 12 2C14.116 2 15.9787 2.74167 17.588 4.225C19.1973 5.70833 20.0013 7.7 20 10.2C20 10.95 19.8543 11.7377 19.563 12.563C19.2717 13.3883 18.8467 14.2467 18.288 15.138C17.7293 16.0293 17.0333 16.95 16.2 17.9C15.3667 18.85 14.4083 19.825 13.325 20.825C13.1417 20.9917 12.9333 21.1167 12.7 21.2C12.4667 21.2833 12.2333 21.325 12 21.325ZM12 12C12.55 12 13.021 11.8043 13.413 11.413C13.805 11.0217 14.0007 10.5507 14 10C13.9993 9.44933 13.8037 8.97867 13.413 8.588C13.0223 8.19733 12.5513 8.00133 12 8C11.4487 7.99867 10.978 8.19467 10.588 8.588C10.198 8.98133 10.002 9.452 10 10C9.998 10.548 10.194 11.019 10.588 11.413C10.982 11.807 11.4527 12.0027 12 12Z"
          fill="#B3B3B3"
        />
      </svg>

      <p>{location}</p>
    </div>
    <div class="price">{price}€</div>
  </div>
</div>

<style lang="scss">
  .advert {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    font-family: "Inter", sans-serif;
  }

  .title {
    font-size: 18px;
    font-weight: 500;
    line-height: 21.78px;
    text-align: left;
    color: #000000;
  }

  .description {
    margin-top: 8px;
    font-size: 18px;
    font-weight: 400;
    line-height: 21.78px;
    text-align: left;
    color: #b3b3b3;
  }

  .date {
    font-size: 18px;
    font-weight: 400;
    line-height: 21.78px;
    text-align: left;
    color: #b3b3b3;
    margin-top: 14px;
  }

  .location {
    font-size: 18px;
    font-weight: 400;
    line-height: 21.78px;
    text-align: left;
    color: #b3b3b3;
    margin-top: 8px;
  }

  .location,
  .date {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    flex-direction: row;
    gap: 5px;
  }

  .price {
    margin-top: 16px;
    font-size: 36px;
    font-weight: 700;
    line-height: 43.57px;
    text-align: left;
    color: #42d392;
  }
</style>
