#import "template.typ": *
#show: ams-article.with(
  title: "Die Anwendbarkeit des zweiten Hauptsatzes der Thermodynamik auf
  ökonomische Systeme: Entwicklung und Vergleich von Modellierungsansätzen",
  authors: (
    (name: "Daniel Meiborg"),
),
  bibliography-file: "refs.yml",
  abstract: [
    This research paper investigates the applicability of the second law of
    thermodynamics to economic systems. Based on the Shannon entropy and
    fundamental concepts of Markov processes, various approaches to modeling
    economic structures are presented and analyzed. These include agent-based
    and diffusion-based Markov processes, entropy-based economic systems,
    evolutionary algorithms, and atomic operations. The advantages and
    disadvantages of each approach are discussed. The paper demonstrates that
    it is possible to apply the second law of thermodynamics to economic
    systems by creating appropriate models and introducing compensating
    terms to satisfy both reality and the second law of thermodynamics.
    Thus the modeling methods presented can help to better understand and
    analyze the underlying mechanisms and relationships in economic systems.
  ],
)

= Einleitung

_You should call it entropy [...] no one really knows what entropy really is,
so in a debate you will always have the advantage._

#align(center)[John von Neumann zu Claude Shannon @tribus1971information[p.~180]]

Die Gültigkeit des zweiten Hauptsatzes der Thermodynamik für physikalische
Prozesse ist allgemein anerkannt und hat weitreichende Implikationen für unser
Verständnis der Welt. Dieser fundamentale Grundsatz besagt, dass in einem
geschlossenen System die Entropie stets zunimmt. Während der zweite Hauptsatz
der Thermodynamik für eine Gruppe von stochastischen Prozessen allgemein
bewiesen werden kann, gibt es bisher noch keine umfassenden Ansätze, dieses
Konzept auch quantitativ auf Wirtschaftssysteme zu übertragen. Daher ist es von
großer Bedeutung, die Anwendbarkeit des zweiten Hauptsatzes der Thermodynamik als
eines der grundlegendsten Prinzipien der Physik auf ökonomische Systeme
genauer zu untersuchen.

Um komplexe Phänomene zu untersuchen, bedient sich die Wissenschaft oft
mathematischer Modelle. In den Wirtschaftswissenschaften gibt es zahlreiche 
Modelle, die dazu dienen, wirtschaftliche Zusammenhänge abzubilden. In dieser
Arbeit werden zunächst die mathematischen Konzepte vorgestellt, die nötig sind,
um den zweiten Hauptsatz der Thermodynamik abstrakt zu formulieren. Darauf
aufbauend werden mehrere Modelle mit dem Zweck, den zweiten Hauptsatz der
Thermodynamik zu erfüllen, formuliert. Die Analyse dieser Ansätze umfasst die
Diskussion der Vor- und Nachteile sowie die Identifikation von Bereichen, in
denen sie potenziell erweitert oder verbessert werden können. 

Die in dieser Arbeit vorgestellten Modellierungsmethoden bieten die Grundlage
für weitere Forschung in diesem Bereich. Die Anwendung der Modelle bleibt 
zukünftiger Entwicklung überlassen.

Die präsentierten Modelle können dazu beitragen, die zugrunde
liegenden Mechanismen und Zusammenhänge in Wirtschaftssystemen
besser zu verstehen und zu analysieren. Dabei kann die Anwendung des zweiten
Hauptsatzes der Thermodynamik auf ökonomische Strukturen neue Perspektiven und
Erkenntnisse liefern, die sowohl für die theoretische Forschung als auch für die
praktische Anwendung von Bedeutung sind.

#pagebreak()

= Theoretische Grundlagen

Um Modelle über den zweiten Hauptsatz der Thermodynamik aufstellen zu können,
ist es unbedingt notwendig, diesen mathematisch beschreiben zu können. Es gilt,
eine abstrakte Definition von Prozessen und Entropie zu benutzen, um die
Grundlage für eine Erweiterung dessen Anwendungsgebietes zu schaffen. In diesem
Kapitel soll ein kurzer Überblick über die Konzepte von Information, Entropie,
Markov-Prozessen und, darauf aufbauend, der formalen Definition des zweiten
Hauptsatzes der Thermodynamik gegeben werden.

== Information

In der Informatik wird der Informationsgehalt, auch genannt Shannon-Information,
als Menge an "Überraschung" eines Ereignisses definiert. Sei $p(x)$ die
Wahrscheinlichkeit des Ereignisses $x$. Dann ist der Informationsgehalt formal
als $I(x) = -log_2(p(x))$ (in Bit) definiert. Hierbei ist zu beachten, dass
diese Größe nichts mit dem im Sprachgebrauch verwendeten Begriff von _Inhalt_ zu
tun haben muss. Der Informationsgehalt sagt nichts darüber aus, wie nützlich die
Nachricht ist. Stattdessen kann diese Größe als die natürliche Grenze für die
Kompression einer Nachricht verstanden werden, oder als das Minimum an Ja-Nein
Fragen, um das Ergebnis zu bestimmen (bei nicht-natürlichen Zahlen die
durchschnittliche Anzahl).

Ein Beispiel: Eine typische faire Münze hat eine Wahrscheinlichkeit von $1/2 $
für Kopf und $1/2 $ für Zahl. Der Informationsgehalt für beide Ereignisse ist
also jeweils $-log_2(1/2) = 1 $ Bit. Bei einer nicht-fairen Münze mit einer
Wahrscheinlichkeit von $1/3 $ für Kopf und $2/3 $ für Zahl ist der
Informationsgehalt für Kopf $-log_2(1/3) approx 1,585 $ Bit und für Zahl
$-log_2(2/3) approx 0,585 $ Bit.

Der Informationsgehalt wird in dieser Arbeit hauptsächlich für die Definition
von Entropie genutzt, mit Ausnahme des Modells in @markup-sprache.

=== Eigenschaften des Informationsgehalts

Der Informationsgehalt ist eine streng monoton sinkende Funktion der
Wahrscheinlichkeit, welche für $(0, 1]$ definiert ist. Der Informationsgehalt
von einem Ereignis, welches völlig sicher ist, also eine Wahrscheinlichkeit von
$1$ hat, ist $0$. Der Informationsgehalt eines Ereignisses, welches nicht
vorkommen kann, wäre $oo$.

Der Informationsgehalt von zwei unabhängigen Ereignissen ist die Summe der
beiden Informationsgehalte.

#pagebreak()

== Entropie

Die Entropie ist ein zentraler Gegenstand des zweiten Hauptsatzes der
Thermodynamik und findet sich in zahlreichen Einzelwissenschaften wieder. In der
Physik findet man sie in Gestalt der quantenmechanisch definierten
Von-Neumann-Entropie, in den Temperaturdifferenzen verschiedener Systeme und in
der statistischen Mechanik. In der Chemie existiert die Reaktionsentropie. Jede
verwendet eine etwas unterschiedliche Definition. In der Informatik wird
hauptsächlich die Entropie nach Claude Shannon verwendet. Für die Zwecke dieser
Arbeit wird ausschließlich die Shannon-Entropie verwendet und ist synonym mit
Entropie zu verstehen.

=== Formale Definition

Die Entropie ist der Erwartungswert des Informationsgehaltes einer zufälligen
Variable. Sie ist definiert als $H(X) = sum_(x in X) p(x) dot I(x) = -sum_(x in X)
p(x) dot log_2(p(x))$ (in Bit), wobei $p(x)$ die Wahrscheinlichkeitsfunktion der
zufälligen Variable ist @gray1990informationtheory[~p.18].

=== Anschauliche Erläuterung

Entropie wird oft als Maß für die Unordnung eines Systems beschrieben. Die
Shannon-Entropie ist allerdings etwas abstrakter: Sie beschreibt die Verteilung
von Wahrscheinlichkeiten.

Die Entropie ist eng mit der statistischen Mechanik verknüpft. In dieser hat ein
Gas, welches gleichmäßig verteilt ist, eine hohe Entropie, während ein Zustand,
in dem alle Gasmoleküle an einer Seite des Raumes sind, eine niedrige Entropie
hat. Das hat den Grund, dass es deutlich mehr Zustände i.e. eine höhere
Wahrscheinlichkeit gibt, dass der erste Zustand eintritt. Dieses Verhältnis
entspricht allerdings eher der Information der Zustände als der Entropie des
Systems.

Nehmen wir wieder das Beispiel mit der Münze: Bei der fairen Münze ist die
Entropie $(1 + 1)/2 = 1$ Bit. Bei der nicht-fairen Münze ist die Entropie $1,585
dot 1/3 + 0,585 dot 2/3 approx 0,918$ Bit. Die Entropie ist also niedriger als bei
der Einheitsverteilung. Diese Tatsache gilt für alle Wahrscheinlichkeiten, d.h.
die Einheitsverteilung hat immer die höchste Entropie. Verteilungen, bei denen
nur ein einziger Zustand möglich ist, haben die niedrigste mögliche Entropie von
$0$. Die Entropie beschreibt also gewissermaßen die Unordnung der
Wahrscheinlichkeiten.

Ein anderes Beispiel: Folgendes Diagramm stellt die Verteilung von Energie in
zwei Körpern dar. Ein Körper besteht hier aus zwei Partikeln, die jeweils eine
Einheit Energie besitzen können. Auf der linken Seite sieht man die Menge an
Energieeinheiten (rot = 2, grün = 1, blau = 0), die die beiden Partikel
besitzen. Rechts sind die möglichen Zustände aufgelistet (Kreuz = hat eine
Einheit Energie). Für eine gleichmäßige Verteilung der Energie zwischen den
beiden Körpern gibt es mehr Möglichkeiten, also eine höhere Entropie (hier 2
Bit). Eine ungleiche Verteilung hat dagegen eine niedrige Entropie (hier 0 Bit).

#figure(
  image("thermodynamics.png"),
  caption: [Verteilung von Energie in zwei Körpern mit jeweils zwei Partikeln, eigene Abbildung]
)

// Nicht korrekt, negative Entropie und Negentropie wird durcheinander gebracht
//=== Negentropie
//
//Negentropie (auch Negative Entropie genannt) ist die Bezeichnung für die Differenz
//zwischen der momentanen Entropie eines Systems und dessen maximalen Entropie.
//Negentropie wird in der Physik als allgemein "nützlich" angesehen und findet
//sich z.B. in den Temperaturdifferenzen in einem Motor wieder. Eine
//Negentropie-Quelle ist ein Teil eines Systems, der eine größere Menge an
//Negentropie besitzt und diese abgeben kann, wie beispielsweise eine Batterie.
//Dieses Konzept ist essenziell für die Ausgleichsterme in @ausgleichsterme.

#pagebreak()

== Markov-Prozesse

In @zweiter-hauptsatz-der-thermodynamik wird gezeigt, dass der zweite Hauptsatz
der Thermodynamik allgemein für Markov-Prozesse definiert werden kann. Aus diesem
Grund basieren fast alle Modelle in dieser Arbeit in der einen oder anderen Form
auf Markov-Prozessen.

=== Definition <markov-prozess-definition>

Ein Markov-Prozess, auch Markov Chain genannt, ist ein stochastischer Prozess,
der die Markov-Eigenschaft besitzt. Er besteht aus einer Menge an Zuständen und
einer Wahrscheinlichkeitsmatrix, die die Übergangswahrscheinlichkeiten zwischen
den Zuständen beschreibt. Die Markov-Eigenschaft besagt, dass die
Übergangswahrscheinlichkeiten ausschließlich von dem aktuellen Zustand abhängen.
Sei A die Übergangsmatrix mit $A_(i j)$ als Wahrscheinlichkeit, dass Zustand $j$
in Zustand $i$ übergeht, und $arrow(P)(t)$ die Wahrscheinlichkeiten der Zustände
zu Zeitpunkt $t$ in Vektorform. Dann lautet die sogenannte Mastergleichung
$arrow(P)(t + 1) = A arrow(P)(t)$ @tolver2016markovchains[~p.15].

#figure(
  image("markov-chain.png"),
  caption: [Ein simpler Markov-Prozess, eigene Abbildung],
)

Für diese Arbeit sollen lediglich endliche diskrete Markov-Prozesse betrachtet
werden, d.h. Prozesse, bei denen es nur endlich viele Zustände gibt und die Zeit
in diskreten Schritten abläuft.

=== Stationäre Verteilung

Die stationäre Wahrscheinlichkeitsverteilung eines Markov-Prozesses ist die
Verteilung $arrow(P)$, die erfüllt, dass $arrow(P) = A arrow(P)$. Die stationäre
Verteilung ist ein Gleichgewichtszustand des Prozesses.

#pagebreak()

== Zweiter Hauptsatz der Thermodynamik <zweiter-hauptsatz-der-thermodynamik>

Für den zweiten Hauptsatz der Thermodynamik existiert keine eindeutige
Formulierung. Alle Formulierungen aber enthalten die folgende Kernaussage:

_Entropie kann in einem geschlossenem thermodynamischen System nicht abnehmen._

Diese Aussage lässt sich bis zu einem gewissen Grad allgemein auf stochastische
Prozesse zurückführen. Die Beweise dafür wurden in diesem Paper geführt
@cover1994processes[p.~98-107], aus dem im Folgendem der Beweis für den zweiten
Hauptsatz der Thermodynamik skizziert ist.

#theorem[$H(mu_n)$ steigt monoton für alle endlichen diskreten Markov-Prozesse,
wenn die Übergangswahrscheinlichkeitsmatrix doppelt stochastisch ist, d.h. 
genau dann, wenn die stationäre Verteilung die Einheitsverteilung ist.]

#proof[Sei $m$ die Anzahl an möglichen Zuständen, $mu_n$ eine
Wahrscheinlichkeitsverteilung zu Zeitpunkt $n$ und $mu$ die stationäre
Verteilung, in diesem Fall die Einheitsverteilung. $D(X || Y)$ ist hier die 
Kullback-Leibler-Divergenz (ein Maß für die Unterschiedlichkeit zweier 
Wahrscheinlichkeitsverteilungen). Dann ist

$D(mu_n || mu)
&= sum_x mu_n(x) dot log_2((mu_n(x))/(1/m)) \
&= -H(mu_n) + log_2(m)$

Da $D(mu_n || mu)$ monoton sinkt, steigt $H(mu_n)$ monoton @cover1994processes[p.~103].
]

Dieser Satz lässt sich wie folgt verstehen: Angenommen, man hat einen
Markov-Prozess, keine Informationen über den vorherrschenden Zustand (allerdings
kennt man die prinzipiell möglichen Zustände und die
Übergangswahrscheinlichkeiten) und führt den Prozess für eine gewisse Zeit aus.
Wenn man danach nicht genauer sagen kann, welche Zustände wie wahrscheinlich
sind, gilt der zweite Hauptsatz der Thermodynamik.

== Zusammenfassung

In diesem Kapitel wurden die theoretischen Grundlagen für Entropie und den
zweiten Hauptsatzes der Thermodynamik dargelegt. Der Informationsgehalt ist als
Menge an "Überraschung" eines Ereignisses definiert. Die Entropie beschreibt den
Erwartungswert des Informationsgehalts einer Zufallsvariablen und wird als Maß für
die Unordnung der Wahrscheinlichkeiten verstanden.

Markov-Prozesse wurden als stochastische Prozesse mit der Markov-Eigenschaft
vorgestellt, bei denen die Übergangswahrscheinlichkeiten zum nächsten Zustand
nur vom aktuellen Zustand abhängen. Die stationäre Verteilung eines solchen
Prozesses beschreibt den Gleichgewichtszustand der
Wahrscheinlichkeitsverteilung.

Der zweite Hauptsatz der Thermodynamik besagt allgemein, dass die Entropie in
einem geschlossenen System nicht abnehmen kann. Es wurde gezeigt, dass dieser
Grundsatz genau dann auch auf endliche diskrete Markov-Prozesse angewendet werden
kann, wenn die stationäre Verteilung die Einheitsverteilung ist beziehungsweise
wenn die Übergangswahrscheinlichkeitsmatrix doppelt stochastisch ist. In solchen
Fällen steigt die Entropie monoton an, was bedeutet, dass man durch das
Fortschreiten der Zeit nicht mehr Information über ein System bekommen kann, als man 
zum Zeitpunkt $0$ bereits hatte.

Man sollte allerdings die Einschränkungen der Markov-Eigenschaft in
@markov-prozess-definition nicht übersehen. Der essenzielle Punkt der Markov-Eigenschaft
ist, dass die Wahrscheinlichkeiten für die nächsten Zustände nur von dem momentanen
Zustand abhängig sind. Das bedeutet aber auch, dass Agenten, die beispielsweise aus der
Vergangenheit lernen, das System deutlich komplizierter machen. Nichtsdestotrotz
lassen sich viele Systeme so vereinfachen, dass sie die Markov-Eigenschaft erfüllen.

Ein Markov-Prozess muss also eine doppelt stochastische 
Übergangswahrscheinlichkeitsmatrix beziehungsweise die Einheitsverteilung als stationäre
Wahrscheinlichkeitsverteilung haben, um den zweiten Hauptsatz der Thermodynamik
zu erfüllen. Um das zu erreichen, wird hier das Konzept von Ausgleichstermen
eingeführt.

= Ausgleichsterme <ausgleichsterme>

Da der zweite Hauptsatz der Thermodynamik nur für Markov-Prozesse gilt, deren 
stationäre Verteilung die Einheitsverteilung ist, müssen die Modelle zwangsläufig
dieses Kriterium erfüllen. Ziel ist es, ein Modell so für den zugrundeliegenden
Sachverhalt zu erstellen, dass dieser Markov-Prozess den zweiten Hauptsatz der
Thermodynamik erfüllt. Nachdem das aber ein Spezialfall von Markov-Prozessen ist,
ist es unwahrscheinlich, dass das bei dem ersten Versuch der Fall ist. Aus diesem
Grund kann man Ausgleichsterme einführen. Diese dienen dazu, das Modell so
anzupassen, dass es sowohl die Realität als auch den zweiten Hauptsatz der
Thermodynamik erfüllt. Diese sind häufig in der Gestalt von Systemen, die eine
niedrige Entropie haben und diese von anderen Systemen aufnehmen können. Für
physikalische Prozesse werden als Quelle für niedrige Entropie beispielsweise
große Temperaturunterschiede wie z.B. in einem Motor benutzt.

#pagebreak()

= Agentenbasierter Markov-Prozess

Bei einem agentenbasierten Markov-Prozess versucht man, ökonomische Agenten
(Staat, Banken etc.) zu modellieren. Agenten sind hier im Sinne der
Spieltheorie als Entitäten, die mit ihrer Umwelt agieren können, zu verstehen.
Der erste Schritt ist hier das Verhalten einzelner Agenten z.B. mithilfe von
Entscheidungsdiagrammen (siehe @beispiel-agenten) in eine Form zu übersetzen,
die sich dann weiterverarbeiten lässt.

Im nächsten Schritt muss dieses Modell dann in einen Markov-Prozess umgewandelt
werden. Eine dazu vom Autor konzipierte und entwickelte Bibliothek ist
Entromatica @meiborg2023entromatica.

Dieser Markov-Prozess lässt sich dann vergleichsweise einfach simulieren. Aus
der daraus zu berechnenden Entropie (siehe @entropie-agenten) oder direkt aus
der Übergangswahrscheinlichkeitsmatrix kann man nun berechnen, ob der zweite
Hauptsatz der Thermodynamik gilt. Falls das nicht der Fall ist, muss man
dementsprechend das Ausgangsmodell sukzessive anpassen.

== Entropie <entropie-agenten>

Die Entropie wird hier anhand der Shannon-Entropie der möglichen Zustände
definiert. Die Wahrscheinlichkeitsverteilung $X_0$ ist zu Beginn bekannt, und
die Entropie wird als Funktion über die Zeit dargestellt. Diesem Prinzip folgen
auch alle anderen auf Markov-Prozessen basierenden hier vorgestellten Modelle.

== Beispiel <beispiel-agenten>

Im Folgenden ist ein Diagramm zu sehen, das vereinfacht mögliche Zustände eines
Kaufprozesses auf der Seite des Käufers und die jeweiligen
Übergangswahrscheinlichkeiten darstellt. ‘Stabil’ bezeichnet hier den
Ausgangszustand. In diesem Szenario entschließt sich der Agent von Zeit zu Zeit,
ein Produkt zu kaufen, und erfährt dabei ab und zu Betrug. Das führt dann dazu,
dass er für eine Zeit lang im Zustand ‘Abbruch’ feststeckt, d.h. aufgrund der
Betrugserfahrung vorsichtiger ist.

#figure(
  image("agenten-basiert.png"),
  caption: [Beispiel eines Modells für agentenbasierte Markov-Prozesse, eigene Abbildung]
)


== Vorteile

Da ein tatsächliches Wirtschaftssystem ebenfalls aus einer Vielzahl an Agenten
besteht, hat dieses System das Potenzial, bei idealer Umsetzung gute Ergebnisse
zu liefern.

== Probleme

Dabei treten jedoch einige Probleme auf: Zunächst einmal ist die Modellierung
von menschlichen Agenten extrem aufwändig, schwierig oder schlicht unmöglich.
Auch bei sehr simplen Systemen muss eine Vielzahl an Parametern willkürlich
gesetzt werden, da die Datenlage und die Verarbeitungskapazität in der Regel
sehr limitiert sind. Des Weiteren sind die Ausgleichsterme in Vergleich zu den
folgenden Ansätzen schwieriger zu implementieren, da sie erfordern, dass
entweder neue Agenten hinzukommen oder bisherige ihre Entscheidungen radikal
verändern, was die Aussagekraft des Modells weiter beeinträchtigt.

#pagebreak()

= Diffusionsbasierter Markov-Prozess

Das Vorgehen bei diffusionsbasierten Markov-Prozessen ist ähnlich wie bei
agentenbasierten Markov-Prozessen. Der Hauptunterschied besteht allerdings
darin, dass anstatt alle Agenten weitestgehend einzeln zu modellieren, lediglich
grobkörnig die Flüsse von Kapital bzw. Ressourcen angegeben werden. Der Begriff
‘Diffusion’ ist hier so zu verstehen, dass Dienstleistungen oder Produkte i.d.R.
eine Form von Gegenleistung bedingen und Kapital bzw. Ressourcen so zwischen
verschiedenen Agenten diffundieren.

== Beispiel

In diesem Diagramm sind schematische Beziehungen zwischen ökonomischen Entitäten
zu sehen. Im nächsten Schritt müssen hier Wahrscheinlichkeiten für Diffusion und
Quantitäten angegeben werden.

#figure(
  image("diffusions-basiert.png"),
  caption: [Beispiel eines Modells für diffusionsbasierte Markov-Prozesse, eigene Abbildung]
)

== Vorteile

Ein wesentlicher Vorteil dieses Ansatzes ist, dass Entscheidungen von Agenten
von Anfang an als nicht vorhersagbar angenommen werden. Stattdessen werden
lediglich die größeren Zusammenhänge beachtet. Tatsächlich ist diese
Unvorhersehbarkeit hier sogar hilfreich, da Zufall für Markov-Prozesse ein
essenzielles Element ist.

== Probleme

Allerdings kann bei dem Versuch, das Wirtschaftssystem auf einer niedrigeren
Ebene zu modellieren, dieses Konzept in eine kompliziertere Version der
agentenbasierten Version ausarten. Es muss also immer zwischen den beiden
Ansätzen abgewogen werden.

#pagebreak()

= Entropiebasiertes Wirtschaftssystem <entropiebasiertes-wirtschaftssystem>

Der nächste Ansatz besteht darin, lediglich diejenigen Teile der Wirtschaft zu
betrachten, die sich  selbst mit Wahrscheinlichkeiten beschäftigen. Dieses
breite Feld umfasst alles von Versicherungen, Glücksspiel bis hin zu
Kryptowährungen. Hier können sowohl reale Systeme betrachtet werden als auch
fiktive Wirtschaftssysteme, in denen beispielsweise reine Information eine
Währung ist.

== Beispiel

Ein Beispiel dafür ist ein virtuelles Pferderennen: In diesem ist die einzige
Information, die das ansonsten geschlossene System (z.B. einen Computer)
verlässt, der (eindeutige) Gewinner. Bei 8 Pferden sind das 3 Bit Information.
Vor Beginn des Zufallsexperiments gibt es also 3 Bit Ungewissheit; danach sind
es 0 Bit (auf das Pferderennen bezogen). Nun kann man einen Markov-Prozess aus
dem Pferderennen und Umgebung erstellen. Um diesen nach dem zweiten Hauptsatz zu
modellieren (der, unseres Wissens nach, auf die Physik zutrifft), muss diese
Pferderennen-Simulation dabei mindestens an 3 Bit Entropie zunehmen. Anhand des
Landauer-Prinzips lässt sich ein Minimum an Energie festlegen, welches benötigt
wird, um ein einzelnes Bit an Information zu löschen. Dieses Minimum wird nach
Landauer @landauer1961limit[p.~188] mithilfe folgender Formel berechnet: $E = k_B T
ln(2)$, wobei $k_B$ der Boltzmann-Konstante entspricht. Ein Bit entspricht bei
Raumtemperatur also mindestens $E = k_B dot 295K dot ln(2) approx 2,823 dot 10^(-21)J$
und die 3 Bit etwa $2,823 dot 10^(-21)J dot 3 dot 8,469 dot 10^(-21)J$, die in Wärme
konvertiert werden müssen. Bei einem Preis von etwa $0,33 (€)/("kWh")$ 2022 in
Deutschland @destatis2022electricity sind das etwa $8,469 dot 10^(-21)J dot 0,33 dot
3,6 dot 10^(-6) €/J approx 10^(-26)€$, die der Ausgang mindestens wert ist.
Information kann man mithilfe von Maxwell's Dämon @bennet1987demon zu nützlicher
Arbeit zurückverwandeln (genauer gesagt das Überschreiben von Speicher). So
könnte man in einer Gesellschaft prinzipiell Information als tatsächlichen
Wertgegenstand einsetzen.

== Vorteile

Da die Branche direkt an Wahrscheinlichkeiten orientiert ist, hat der Begriff
der Entropie hier mehr Aussagekraft als bei den generischen agenten- oder
diffusionsbasierten Modellen.

== Probleme

Allerdings wird für reelle Wirtschaftszweige deutlich, dass die Entropie, die
man bei der Spekulation erhält, in aller Regel nicht dem entspricht, was eine
solche Berechnung ergeben würde, da solche Systeme nicht vollständig
abgeschlossen sind und Information nach außen dringen kann.

#pagebreak()

= Markup-Sprache <markup-sprache>

Während die bisherigen Modellierungsmöglichkeiten darauf beruhen, ein bestimmtes
Wirtschaftssystem zu modellieren und dessen Entropie zu berechnen, wird bei
diesem Ansatz gewissermaßen die Entropie aller möglichen Wirtschaftssysteme
berechnet.

Im ersten Schritt wird dazu eine Markup-Sprache definiert. Das kann
beispielsweise ein XML-Dialekt sein. Diese Syntax wird dazu verwendet, ein
bestimmtes System zu beschreiben.

Es ist wichtig zu beachten, dass zunächst nur der Informationsgehalt, nicht die
Entropie dieses Dokuments berechnet wird. Dazu wählt man eine gewisse Länge für
die Dokumente oder begrenzt die Anzahl anderweitig. Danach weist man jedem
dieser Dokumente eine Wahrscheinlichkeit zu. Das kann jeweils die gleiche sein
oder komplexere Ansätze wie in @evolutionäre-algorithmen illustriert. Diese
Wahrscheinlichkeit lässt sich, wie in den theoretischen Grundlagen erläutert,
dann in den Informationsgehalt umwandeln. Schließlich kann man für die ganze
Menge der Dokumente die Entropie berechnen.

== Beispiel

Im Folgenden ist eine mögliche Darstellung eines aus drei Agenten bestehenden
Systems in einem frei erfundenen XML-Dialekt zu sehen.

```xml
<economy>
  <agents>
    <agent id="A1" type="producer" />
    <agent id="A2" type="consumer" />
    <agent id="A3" type="producer" />
  </agents>
  <products>
    <product id="P1" producer="A1" />
    <product id="P2" producer="A3" />
  </products>
  <connections>
    <trade producer="A1" consumer="A2" product="P1" price="10" />
    <trade producer="A3" consumer="A2" product="P2" price="5" />
  </connections>
</economy>
```

== Vorteile

Der wesentliche Vorteil hier ist, dass man die Entropie der _Struktur_ der
Wirtschaftssysteme berechnen kann, anstatt die Entropie ihres Verhaltens.
Dadurch muss man nicht den schwierigen Schritt gehen und die einzelnen Agenten
simulieren.

== Probleme

Allerdings kann ohne weitere Modifikationen der 2. Hauptsatz der Thermodynamik
nicht angewendet werden, da die Dokumente keine Markov-Prozesse sind.

#pagebreak()

= Evolutionäre Algorithmen <evolutionäre-algorithmen>

Um die Markup-Sprache auch mit der Zeit in Verbindung zu setzen, kann man diese in
evolutionäre Algorithmen einbetten. Eine Möglichkeit dafür besteht in der
Mutation der Wirtschaftssysteme. So ein Modell benötigt im Wesentlichen folgende
Elemente: Ein Datenformat (hier die Markup-Sprache) für die Kandidaten (ein
einzelnes Dokument), eine Funktion, die die Fitness eines Kandidaten angibt (das
kann zum Beispiel die Kaufkraftparität des Bruttoinlandsprodukts pro Kopf oder
der Gini-Koeffizient (Vermögensverteilung) sein) und einen Algorithmus für die
Mutation.

Beim Ausführen der Simulation generiert man zunächst eine zufällige Menge an
Kandidaten. Eine Iteration besteht darin, zunächst eine Form von Selektion
anzuwenden, beispielsweise die nach der Fitnessfunktion bestimmten obersten $x%$
auszuwählen. Diese werden anschließend mutiert. Das kann zufällig sein oder
komplexere Methoden verwenden, in denen (simuliert) die Agenten das
Wirtschaftssystem anpassen.

Man kann die Entropie berechnen, indem man anhand der Fitness der Kandidaten
ihren Anteil an der Population berechnet, und daraus wiederum die
Wahrscheinlichkeit, diesen bei einer zufälligen Auswahl auszuwählen. Aus dieser
Wahrscheinlichkeitsverteilung kann man dann die Entropie berechnen.

== Vorteile

Durch diesen Ansatz kann man die Markup-Sprachen ebenfalls als Modell über die
Zeit betrachten. Das erleichtert den Vergleich mit den anderen Modellen, welche
oft Zeit als Parameter beinhalten.

== Probleme

Allerdings stellen diese Simulationen ebenfalls keinen Markov-Prozess dar. Das
liegt daran, dass man höchstens die Verteilung der gesamten Population als
Zustand betrachten könnte und der Markov-Prozess dann vollkommen deterministisch
wäre - also uninteressant.

Des Weiteren tendiert ein evolutionäre Algorithmus dazu, einen oder wenige
Kandidaten auszuwählen, welche dann die gesamte Population stellen. Das steht im
Kontrast zu der Initialverteilung, wo es relativ viele verschiedene Kandidaten
gibt - die Entropie sinkt also. Allerdings ist auch das nicht immer gegeben, da
durch Mutation die Verteilung sich mindestens kurzfristig verbreitern kann.

#pagebreak()

= Atomare Operationen

Im Gegensatz zu den bisherigen Modellen werden hier Prozesse auf einer möglichst
niedrigen Ebene betrachtet. Anstatt die Struktur eines Wirtschaftssystems zu
analysieren, versucht man, einfache ökonomische Prozesse mit Markov-Prozessen zu
modellieren. ‘Atomar’ ist hier nicht im Sinne der Kernphysik zu verstehen,
sondern vielmehr als _unteilbare_ Handlungen.

== Beispiel

Eine solche atomare Operation könnte zum Beispiel die Produktion von Gütern aus
Rohstoffen sein. In dem folgenden Diagramm kann man einen solchen Prozess sehen.
Es gibt in diesem Markov-Prozess zwei Gruppen von Zuständen: _Rohstoff_ und
_Produkt_. Zum Zeitpunkt $t=0$ ist unbekannt, welcher der drei möglichen
Zustände R1, R2 oder R3 von dem Rohstoff tatsächlich eingenommen wird, d.h. alle
haben eine Wahrscheinlichkeit von $1/3$ und damit die Entropie $H(X_0) =
log_2(3)$. Zum Zeitpunkt $t=1$ wechseln diese Zustände zufällig entweder zu P1
oder P2. Die Entropie ist somit $H(X_0) = log_2(2) = 1 "Bit"$.

#figure(
  image("atomare-operationen-basis.png"),
  caption: [Beispiel eines Modells für atomare Operationen, eigene Abbildung]
)

Um den zweiten Hauptsatz der Thermodynamik zu erfüllen, kann man nun das Modell
auf mehrere Arten erweitern. Eine solche Möglichkeit ist die Einführung von
Energie als Edukt (z.B. in Form von Photonen). Hier ist es wichtig zu beachten,
dass R1, R2, R3 und E1 keine getrennten Zustände sind, die die Edukte einnehmen
können, sondern ein konkreter Zustand des Markov-Prozesses aus der Kombination
(R1, E1), (R2, E1) oder (R3, E1) besteht. Dementsprechend bleibt die
Gesamtentropie von $X_0$ gleich. Basis für die Annahme der geringen Anzahl an
Zuständen der ‘Energie’ ist, dass es weniger Zustände beispielsweise für
Photonen gibt (um genau zu sein $H = k(1-ln(f_r))$; $f_r$ entspricht der
Photonenverteilung und $k$ der Boltzmann-Konstante
@kirwan2004photons[p.~725-734]) als für eine bestimmte Menge, z.B. eines idealen
Gases. Die Zugabe von Energie bedingt i.d.R. eine höhere Quantität an Produkten.
In diesem Beispiel wird angenommen, dass sich die Anzahl an Produktteilen
dadurch verdoppelt. Da es für einen einzelnen Teil bereits 2 Zustände gibt, gibt
es für den Komplex aus beiden somit 4 mögliche Zustände: (P1_1, P2_1), (P1_1,
P2_2), (P1_2, P2_1), (P1_2, P2_2). Die Entropie beträgt damit $H(X_1) = log_2(4)
= 2 "Bit"$.

#figure(
  image("atomare-operationen-energie.png"),
  caption: [Beispiel eines Modells für atomare Operationen, Produktion mit Energie, eigene Abbildung]
)

Eine andere Möglichkeit ist Abfall. Dabei wird angenommen, dass bei der
Produktion eine gewisse Menge an undefinierbarem Abfall entsteht. Undefinierbar
meint hier, dass man bei der Produktion nicht darauf achtet, eine bestimmte Art
von Abfall zu produzieren, sondern der Abfall irgendetwas sein kann. Somit hat
dieser hier eine hohe Anzahl an möglichen Zuständen (willkürlich gewählt 5). Das
Entropieverhältnis ist hier also $H(X_0) = log_2(3) approx 1,585 "Bit"$ und
$H(X_1) = log_2(2 dot 5) = log_2(2) + log_2(5) approx 3,322 "Bit"$.

#figure(
  image("atomare-operationen-müll.png"),
  caption: [Beispiel eines Modells für atomare Operationen, Produktion mit Abfall, eigene Abbildung]
)

#pagebreak()

== Vorteile

Im Gegensatz zu den anderen Modellen ist dieser Ansatz vergleichsweise
anschaulich und nachvollziehbar. Des Weiteren muss man dazu nicht das Verhalten
von Agenten voraussagen, was die Umsetzbarkeit dramatisch erhöht.

== Probleme

Allerdings wächst die Komplexität dieses Ansatzes bei Betrachtung eines größeren
Systems immens. Hier verschwimmen die Grenzen zu den ersten beiden Modellen.

= Zusammenfassung

Agenten- und diffusionsbasierte Modelle bieten geeignete Ansätze, um die
Entropie des Verhaltens eines einzelnen Wirtschaftssystems zu analysieren und
ökonomische Prozesse auf einer höheren Ebene zu betrachten. Jedoch sind diese
Modelle aufgrund ihrer Komplexität schwer zu interpretieren und zu skalieren.
Für den Vergleich der Struktur eines Wirtschaftssystems ist hingegen eine
Markup-Sprache eher geeignet, da dabei die Agenten nicht simuliert werden
müssen. Schließlich ermöglichen atomare Operationen, ökonomische Prozesse auf
einer niedrigen Ebene zu betrachten.

#pagebreak()

= Limitationen

Während die Probleme der einzelnen Modellierungsansätze bereits in deren Kapitel
vorgestellt wurden, gilt es dennoch, auch die allgemeinen Grenzen der Anwendung
des zweiten Hauptsatzes der Thermodynamik auf Wirtschaftssysteme zu diskutieren.

Ein grundlegendes Problem besteht darin, dass die Modelle auf
mitunter realitätsfernen Annahmen basieren. Das betrifft vor allem die
Ausgleichsterme, die zur Anpassung der Modelle an den zweiten Hauptsatz der
Thermodynamik verwendet werden. Diese Ausgleichsterme können willkürlich wirken
und stellen somit eine mögliche Schwachstelle in der Modellierung dar. Das gilt
insbesondere für die künstliche Erschaffung von negativen Entropiequellen.

Die Skalierbarkeit und Komplexität der Modelle stellen ebenfalls eine Limitation
dar. Die Anwendung des zweiten Hauptsatzes der Thermodynamik auf
Wirtschaftssysteme erfordert die Entwicklung von komplexen Modellen und
Simulationen, die sowohl die zugrunde liegenden Prozesse als auch den
zweiten Hauptsatz der Thermodynamik berücksichtigen. Diese Komplexität kann dazu
führen, dass die Modelle schwierig zu interpretieren sind. Insbesondere bei der
Anwendung von agenten- und diffusionsbasierten Modellen können die
resultierenden Simulationen schwierig zu analysieren sein.

Die begrenzten Ressourcen, die zur Verfügung stehen, stellen eine weitere
Problematik dar. Die Erstellung und Simulation der Modelle erfordert sowohl
ausreichend Zeit als auch Wissen über die zugrunde liegenden Prozesse und
Gesetze. Darüber hinaus sind die Rechenleistung und die technischen Ressourcen,
die für die Durchführung von Simulationen und die Analyse der Ergebnisse
erforderlich sind, begrenzt. Diese limitierten Ressourcen können dazu
führen, dass die Modelle und Simulationen möglicherweise nicht in der
erforderlichen Tiefe untersucht und analysiert werden können.

Schließlich ist zu beachten, dass Wirtschaftssysteme in den meisten
Wirtschaftstheorien ein Gleichgewicht anstreben. Dies bedeutet, dass die
Entropie in einem Wirtschaftssystem stark abfällt. Das kann die Einführung von
Ausgleichstermen verkomplizieren, da man mitunter keinen direkt ökonomischen
Faktor verwenden kann, sondern auf beispielsweise die Umweltauswirkungen
zurückgreifen muss.

#pagebreak()

= Ausblick

Die in dieser Arbeit vorgestellten Ansätze zur Anwendung des zweiten Hauptsatzes
der Thermodynamik auf ökonomische Systeme bieten ein vielversprechendes
Potenzial für zukünftige Forschung und Anwendung in der Praxis. Im Folgenden
werden einige Perspektiven und weitere weiterführende Forschungsansätze diskutiert,
die sowohl die Umsetzung der Ansätze in die Praxis als auch die Modellierung von
Risikomanagement und die Anwendung künstlicher Intelligenz für die Erstellung
der Modelle betreffen.

== Umsetzung der Ansätze

Nach der Entwicklung der Modellierungsansätze liegt die tatsächliche
Modellerstellung nahe. Um die Effektivität dieser Modelle in
realen Wirtschaftssystemen zu untersuchen, ist es erforderlich, sie anhand von
empirischen Daten zu validieren und zu kalibrieren. Hierbei können
beispielsweise historische Daten aus verschiedenen Wirtschaftsbranchen und
Ländern herangezogen werden, um die Modelle zu testen und zu optimieren.
Die Effektivität der Umsetzung hängt dabei unter anderem von der Menge und
Aussagekraft der verfügbaren Daten ab.

== Künstliche Intelligenz

Hilfreich für die Erstellung der Modelle nach den oben beschriebenen Ansätzen
kann auch die Verwendung von künstlicher Intelligenz (KI), insbesondere von Deep
Learning und evolutionären Algorithmen (nicht zu verwechseln mit dem Modell 
"Evolutionäre Algorithmen" in @evolutionäre-algorithmen). Diese Technologien
könnten dazu verwendet werden, die Anpassung der Modelle mithilfe der
Ausgleichsterme möglichst realitätsnah zu gestalten.

Die Integration von KI in die vorgestellten Ansätze zur Modellierung von
ökonomischen Systemen unter Berücksichtigung der Entropie könnte somit dazu
beitragen, die Effizienz und Anwendbarkeit dieser Modelle in der Praxis weiter
zu erhöhen und gleichzeitig die Komplexität der Modelle besser zu bewältigen.

== Modellierung von Risikomanagement

Die Anwendung des zweiten Hauptsatzes der Thermodynamik auf ökonomische Prozesse
eröffnet auch neue Möglichkeiten für die Modellierung von Risikomanagement in
Unternehmen und Finanzmärkten. Da Information dabei eine essenzielle Rolle
spielt, liegt es nahe, den Zusammenhang mit dem zweiten Hauptsatz der
Thermodynamik genauer zu untersuchen.

Zukünftige Forschungsarbeiten könnten sich darauf konzentrieren, die
vorgestellten Modelle zur Analyse von Risiken in verschiedenen Marktsegmenten
und Branchen zu adaptieren und weiterzuentwickeln. Hierbei könnten
beispielsweise die Volatilität von Aktienkursen, Wechselkursrisiken oder
Kreditrisiken im Zusammenhang mit der Entropie untersucht werden. Diese Risiken
können dann mit Entropie und Information verknüpft werden (in Anlehnung an
die entropiebasierten Wirtschaftssysteme in @entropiebasiertes-wirtschaftssystem).

= Fazit

In dieser Arbeit wurde die Anwendbarkeit des zweiten Hauptsatzes der
Thermodynamik auf ökonomische Systeme untersucht. Es wurden verschiedene
Modellierungsansätze vorgestellt, die es ermöglichen, den zweiten Hauptsatz der
Thermodynamik auf Wirtschaftssysteme zu übertragen und somit einen neuen Blick
auf ökonomische Zusammenhänge zu werfen. Dabei wurden agenten- und
diffusionsbasierte Modelle, Markup-Sprachen, Entropie-basierte Systeme,
evolutionäre Algorithmen und atomare Operationen untersucht und miteinander
verglichen.

Die Analyse der verschiedenen Ansätze zeigt, dass es möglich ist, den zweiten
Hauptsatz der Thermodynamik auf Wirtschaftssysteme anzuwenden. Die vorgestellten
Modellierungsmethoden bieten eine Grundlage für weitere Forschung in diesem Bereich
und können dazu beitragen, die zugrunde liegenden Mechanismen und Zusammenhänge in
Wirtschaftssystemen besser zu verstehen und zu analysieren.

#pagebreak()