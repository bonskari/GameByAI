# Juuson pelit

Tämä haara on Juuson selainpelien kehitystä varten. Alkuperäinen GameByAI-projekti säilyy main-haarassa.

## Hylkeen meriseikkailu

Pelin tiedostot ovat kansiossa `juusanpelit/` (vanha tekninen kansionimi). `index.html` sisältää pelin ja `version.json` kertoo julkaistun version. Hylje kerää merilevää, väistää haita ja sitä ohjataan peliohjaimella, näppäimistöllä tai kosketuksella.

## Kehitysohjeet tekoälylle

- Kehitä aina tätä samaa peliä; älä luo uutta erillistä peliä.
- Säilytä peliohjaintuki (Gamepad API) ja Android-selaintuki.
- Kun pelin koodi muuttuu, kasvata `version.json`-tiedoston versionumeroa ja vastaavaa `VERSION`-arvoa pelin HTML-tiedostossa. Päivitystarkistus vertailee näitä.
- Jos versiota ei ole muutettu, selain ei saa latautua uudelleen. Uusi versio voidaan ottaa käyttöön heti.
- Älä poista main-haaran vanhaa projektia tai muuta sen sisältöä.
- Julkaise vain pelin tiedostot. Julkaisutyönkulku sijaitsee `.github/workflows/juusanpelit-pages.yml` ja on päivitettävä seuraamaan tätä haaraa.

## Julkaisu

GitHub Pagesin käyttöönotto on tarkistettava repositorion asetuksista. Pelin toimivaa julkista osoitetta ei ole vielä vahvistettu.
