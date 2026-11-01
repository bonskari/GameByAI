# Juuson pelit

Tämä on Juuson pelien oma kehityshaara `juuson-pelit` repositoriossa `bonskari/GameByAI`. Älä muuta haaran nimeä tai `main`-haaraa.

## Pelaaminen

Hylkeen meriseikkailu: hylje kerää merilevää ja väistelee haita. Android-tabletilla peliä ohjataan peliohjaimen vasemmalla tatilla tai ristiohjaimella; myös kosketus ja näppäimistö toimivat.

**Pysyvä peliosoite (GitHub Pages):** https://bonskari.github.io/GameByAI/juuson-pelit/

Peli julkaistaan GitHub Pagesin asetuksella **GitHub Actions** (`.github/workflows/pages.yml`): jokainen push haaraan `juuson-pelit` julkaisee koko repon juuren automaattisesti, ja päällekkäiset julkaisut jonotetaan. Kansiossa `juuson-pelit/` ovat `index.html` ja `version.json`. GitHub Pages -julkaisun valmistumista ja linkin toimivuutta on seurattava erikseen.

## AI-kehitysohjeet

1. Muokkaa aina olemassa olevaa `juuson-pelit/index.html`-tiedostoa; älä tee uutta peliä tai muuta GitHub-haaran nimeä.
2. Säilytä peliohjaimen Gamepad API -tuki, Android-selaintuki, hylje, hait ja kerättävä merilevä.
3. Muuta **jokaisen peliin vaikuttavan päivityksen** yhteydessä sekä HTML:n `VERSION` että `juuson-pelit/version.json`-tiedoston `version` samaan uuteen arvoon. Älä muuta versiota, ellei pelissä ole oikeasti muutoksia.
4. Peli tarkistaa `version.json`-tiedoston minuutin välein välimuistin ohi. Jos versio ei muutu, peli ei lataudu uudelleen. Jos muuttuu, selain avaa uuden version automaattisesti; pelitilannetta ei tarvitse säilyttää.
5. **Pakollinen jokaiselle pelille ja sivulle:** automaattinen julkaisu. Älä poista tai muuta `.github/workflows/pages.yml`-tiedostoa. Jokainen `juuson-pelit/`-kansioon lisätty tai muutettu tiedosto julkaistuu pushilla itsestään osoitteeseen `https://bonskari.github.io/GameByAI/juuson-pelit/<tiedosto>`. Uutta peliä luotaessa tämä ja automaattinen version tarkistus (kohta 4) ovat aina mukana.
6. Säilytä `main`-haaran GameByAI-projekti muuttumattomana. Työskentele vain `juuson-pelit`-haarassa.

## Tiedostot

- `juuson-pelit/index.html`: koko peli.
- `juuson-pelit/version.json`: päivitystarkistuksen versionumero.
- `.github/workflows/pages.yml`: automaattinen Pages-julkaisu.
- `README.md`: projektin ohjeet ihmisille ja tekoälylle.
