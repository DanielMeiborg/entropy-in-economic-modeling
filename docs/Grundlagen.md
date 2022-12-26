---
title: Theorie SIA-Arbeit
subtitle: ENTROPIE EINES WIRTSCHAFTSSYSTEMS
author: Daniel Meiborg
date: \today
bibliography: [./main.bib]
csl: ieee-with-url.csl
link-citations: true
documentclass: article
fontsize: 12pt
---

\maketitle
\newpage
\tableofcontents
\newpage

> _You should call it entropy [...] no one really knows what entropy really is,
> so in a debate you will always have the advantage._

John von Neumann zu Claude Shannon, _Scientific American Vol. 225 No. 3, (1971)_

# Entropie

## Arten und Anwendungen von Entropie

Entropie findet sich auf viele verschiedene Arten der Wissenschaft wieder. In
der Physik findet man sie in Gestalt der quantenmechanisch definierten
Von-Neumann-Entropie, in den Temperaturdifferenzen verschiedener Systeme und in
der statistischen Mechanik, von denen jede eine etwas andere Definition
verwendet. In der Informatik wird hauptsächlich die Entropie nach Claude Shannon
verwendet. Für die Zwecke dieser Arbeit wird ausschließlich die Shannon-Entropie
verwendet und ist synonym mit Entropie zu verstehen.

## Information

In der Informatik wird der Informationsgehalt, auch genannt Shannon-Information,
als Menge an "Überraschung" eines Ereignisses definiert. Formal ist der
Informationsgehalt eines Ereignisses als $I(x) = -\log_2(p(x))$ (in Bit)
definiert. Hierbei ist zu beachten, dass diese Größe nichts mit dem im
Sprachgebrauch verwendeten Begriff von Information zu tun haben muss. Der
Informationsgehalt sagt nichts darüber aus, wie nützlich die Nachricht ist.
Stattdessen kann diese Größe als die natürliche Grenze für die Kompression einer
Nachricht verstanden werden.

Ein Beispiel: Eine typische faire Münze hat eine Wahrscheinlichkeit von $1/2$
für Kopf und $1/2$ für Zahl. Der Informationsgehalt für beide Ereignise ist also
jeweils $-\log_2(1/2) = 1$ Bit. Bei einer nicht-fairen Münze mit einer
Wahrscheinlichkeit von $1/3$ für Kopf und $2/3$ für Zahl ist der
Informationsgehalt für Kopf $-\log_2(1/3) = 1,585$ Bit und für Zahl
$-\log_2(2/3) = 0,585$ Bit.

## Eigenschaften des Informationsgehalts

Der Informationsgehalt ist eine streng monoton sinkende Funktion der
Wahrscheinlichkeit, welche für $(0, 1]$ definiert ist. Der Informationsgehalt
von einem Ereignis, welches völlig sicher ist, also eine Wahrscheinlichkeit von
$1$ hat, ist $0$. Der Informationsgehalt von einem Ereignis, welches nicht
vorkommen kann, wäre $\infty$.

Der Informationsgehalt von zwei unabhängigen Ereignissen ist die Summe der
beiden Informationsgehalte. Abhängigkeit der beiden Ereignisse reduziert die
Informationsgehalt. Diese Reduktion kann auch als Maß für die Abhängigkeit der
Ereignisse verwendet werden.

## Formale Definition Shannon-Entropie

Die Entropie ist der durchschnittliche Informationsgehalt einer zufälligen
Variable. Sie ist definiert als $H(X) = \sum_{x \in X} p(x) \cdot I(x)$ (in
Bit), wobei $X$ die Wahrscheinlichkeitsfunktion der zufälligen Variable ist.

## Anschauliche Erläuterung

Entropie wird oft als Maß für die Unordnung eines Systems beschrieben. In der
Physik kann dieses System aus mehreren Objekten bestehen, welche jeweils eine
Temperatur haben. Die Shannon-Entropie ist allerdings etwas abstrakter: Sie
beschreibt das Maß an Gleichverteilung von Wahrscheinlichkeiten.

Nehmen wir wieder das Beispiel mit der Münze: Bei der fairen Münze ist die
Entropie $\frac{1+1}{2} = 1$ Bit. Bei der nicht-fairen Münze ist die Entropie
$\frac{1,585 \cdot 1/3 + 0,585 \cdot 2/3}{2} \approx 0,459$ Bit. Die Entropie
ist also niedriger als bei der Einheitsverteilung. Diese Tatsache gilt für alle
Wahrscheinlichkeiten i.e. die Einheitsverteilung hat immer die höchste Entropie.
Die niedrigste Entropie haben Verteilungen, be denen nur ein einziger Zustand
möglich ist.

# Markov-Prozesse

# Wann erfüllen Markov-Prozesse den zweiten Hauptsatz der Thermodynamik?

# Wirtschaftssysteme

# Quellen
