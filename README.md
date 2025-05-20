# Tīmekļa tirdzniecības platforma  

## Projekta apraksts  
“Tīmekļa tirdzniecības platforma” ir moderna vietne sludinājumu publicēšanai, meklēšanai un pārvaldīšanai. Gala produkts ļauj reģistrētiem lietotājiem:
- publicēt un rediģēt sludinājumus ar foto, aprakstu, cenu un ģeolokāciju;  
- precīzi meklēt un filtrēt piedāvājumu sarakstu;  
- veidot čatus un veikt darījumus reāllaikā;  
- pārvaldīt favorītus un rakstīt atsauksmes.  

Visas servisa komponentes ir konteinerizētas ar Docker un savienotas, izmantojot Docker Compose.

## Izmantotās tehnoloģijas  
Projektā izmantots:  
- **Front-end:** HTML5, CSS3, TypeScript, Svelte, Vite  
- **Back-end:** Rust (Actix-Web), GraphQL (async-graphql)  
- **Datu glabāšana un kešēšana:** PostgreSQL, Redis  
- **Izvietošana:** Docker, Docker Compose, Nginx, CertBot  
- **Rīki:** Git, GitHub, Visual Studio Code  


## Izmantotie avoti  
1. [Actix-Web Documentation, 2024](https://actix.rs/docs/) — servera ietvara specifikācija.  
2. [GraphQL Documentation, 2024](https://graphql.org/learn/) — API vaicājumu definēšana.  
3. [MDN Web Docs: HTML5, 2024](https://developer.mozilla.org/en-US/docs/Web/HTML) — HTML atbalsts.  
4. [MDN Web Docs: CSS3, 2024](https://developer.mozilla.org/en-US/docs/Web/CSS) — stila lapas.  
5. [PostgreSQL Documentation, 2024](https://www.postgresql.org/docs/) — relāciju datu bāze.  
6. [Redis Documentation, 2024](https://redis.io/docs/) — atmiņas datu glabāšana.  
7. [Svelte Documentation, 2024](https://svelte.dev/docs) — front-end komponenšu ietvars.  
8. [TypeScript Documentation, 2024](https://www.typescriptlang.org/docs/) — statiskā tipizācija.  
9. [Vite Documentation, 2024](https://vitejs.dev/guide/) — izstrādes serveris un bundling.  

:exclamation: *Obligāti jānorāda visi avoti, kas izmantoti kodam vai dokumentācijai. Ja atrod kodu, kas sakrīt ar citu autoru darbu, tas nederēs par pašu izstrādātu risinājumu.*

## Uzstādīšanas instrukcijas  
1. **Priekšnosacījumi:**  
   - [Docker](https://www.docker.com/get-started) un Docker Compose uz Linux, macOS vai Windows.  

2. **Repo klonēšana:**  
   ```bash
   git clone https://github.com/rvtprog-kvalifikacija-20/....git
   cd ss.lv
   docker compose up -d --build --force-recreate
   ```
