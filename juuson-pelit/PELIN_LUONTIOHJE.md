# Uuden pelin luontiohje

Tämä ohje koskee jokaista uutta peliä.

1. Luo uusi peli omana HTML-tiedostonaan `juuson-pelit/`-kansioon GitHubin `juuson-pelit`-haarassa. Älä muuta vanhoja pelejä.
2. Käytä versionumerointia alkaen `1.0.0`; näytä versio ruudulla ja kasvata numeroa joka julkaisussa.
3. **Pakollinen ominaisuus: automaattinen päivitys.** Pelin tulee tarkistaa julkaistun HTML:n versio säännöllisesti (esim. 5 sekunnin välein), käyttää `cache: 'no-store'` ja aikaleimallista URL-parametria, sekä ladata uusi versio automaattisesti version muuttuessa. Tarkista päivitys myös, kun välilehti palaa näkyviin. Käyttäjän ei pidä joutua päivittämään sivua käsin tavallisissa päivityksissä.
4. Lisää automaattinen päivitystarkistus **heti ensimmäiseen julkaisuun**, jotta seuraavat versiot päivittyvät jo auki olevaan peliin. Ensimmäisen kerran vanhasta versiosta ilman päivitystarkistusta siirtyminen voi vaatia yhden uudelleenavauksen.
5. Varmista GitHub Pages -julkaisu ja testaa toimiva URL ennen kuin ilmoitat linkin toimivaksi. Älä arvaa osoitteita tai käytä puheessa osoitteen sijasta 'kautta'-sanoja.
6. Säilytä sama julkinen osoite ja saman HTML-tiedoston polku kaikissa jatkopäivityksissä.
7. Jos käyttäjä on pelaamassa, vältä tarpeettomia automaattisia uudelleenlatauksia: päivitä vain version muuttuessa; huomioi tallennustila tarvittaessa.

Huom: GitHub Pagesin julkaisuviive ja selaimen verkkoyhteys voivat viivästyttää päivitystä; täysin välitöntä päivitystä ei voi luvata.

## Kaikkien pelien pakolliset perusominaisuudet

- **Peliohjain yksin riittää.** Koko peli, valikot, aloitus, tauko, jatkaminen, uudelleenaloitus ja mahdolliset vahvistukset toimivat gamepadilla. Älä lisää ruudulle kosketus- tai ohjauspainikkeita. Testaa gamepad-käyttö kokonaisuudessaan.
- **Automaattinen päivitys ensimmäisestä versiosta lähtien.** Käytä version tarkistusta ja välimuistin ohitusta. Älä edellytä manuaalista sivun päivittämistä tavallisissa päivityksissä. Päivitys tulee samasta pysyvästä osoitteesta.
- **Selkeä versionumerointi.** Aloita 1.0.0:sta, nosta versionumeroa jokaisella julkaistavalla muutoksella, näytä versio pelissä ja tarkista julkaistun version numero.
- **Täsmällinen, klikattava pelilinkki.** Tarkista GitHubin ainoa oikea haara `juuson-pelit`, tiedostopolku ja julkinen URL. Kirjoita osoite oikein Markdown-linkkinä; älä korvaa vinoviivoja puheessa sanoilla. Älä väitä linkkiä toimivaksi ilman tarkistusta.
- **Kuusivuotiaalle sopiva teksti.** Kaikki pelin näkyvät ohjeet, valikot, ilmoitukset ja tarina käyttävät hyvin lyhyitä, selkeitä suomenkielisiä lauseita. Ei pitkiä kappaleita eikä vaikeita termejä.
- **Julkaisun laadunvarmistus.** Tee vain yksi julkaisu kerrallaan. Tarkista, ettei edellinen deployment ole kesken, varmista onnistuminen ja vertaa julkisen HTML:n versiota GitHubin tiedostoon. Älä sekoita haaroja tai julkaisutapoja.

- **Grafiikka:** Suosi aiempaa realistisempaa, yksityiskohtaista ja yhtenäistä pelimaailmaa. Älä rakenna hahmoja, kiviä tai ympäristöä pelkistä yksinkertaisista geometrisista muodoista. Käytä tarvittaessa laadukkaita piirros- tai kuvapohjaisia spritejä, tekstuureja, varjostusta ja animaatiota. Grafiikan tulee silti olla selkeää ja kuusivuotiaalle sopivaa sekä toimia tabletilla sujuvasti.
