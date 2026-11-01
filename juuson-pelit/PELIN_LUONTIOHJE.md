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
