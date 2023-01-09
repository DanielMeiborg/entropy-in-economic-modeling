---
transition: "slide"
theme: "white"
slideNumber: false
enableMenu: false
enableChalkboard: false
---

<script src="https://cdn.jsdelivr.net/npm/reveal.js-mermaid-plugin@1.0.0/plugin/mermaid/mermaid.js"></script>

# Entropie eines Wirtschaftssystems {.r-fit-text}

Theorethische Grundlagen

<figure style="font-size: 20px">
  <blockquote>
    <p>
      You should call it entropy [...] no one really knows what entropy really is, so in a debate you will always have the advantage.
    </p>
  </blockquote>
  <figcaption style="font-size: 15px">
    John von Neumann zu Claude Shannon, Scientific American Vol. 225 No. 3, (1971)</i>
  </figcaption>
</figure>


---

## Information


'Überraschung' eines Ereignisses

`$$I: \Omega \to \mathbb{R}_{\geq 0}$$` {.fragment}

`$$A, B \in \Omega: p(A) < p(B) \implies I(B) < I(A)$$` {.fragment}

`$$A \in \Omega: p(A) = 1 \implies I(A) = 0$$` {.fragment}

`$$A \in \Omega: p(A) \approx 0 \implies I(A) \approx \infty$$` {.fragment}

`$$A \perp\!\!\!\perp B \implies I(A \cap B) = I(A) + I(B)$$` {.fragment}

`$$I(x) \mapsto - \log_2p(x)$$` {.fragment}

---

### Beispiel

Sei `$\Omega = \{ K, Z \}$`

`$$p(K) = \frac{1}{2}; p(Z) = \frac{1}{2}$$`

`$$\implies I(K) = 2; I(Z) = 2$$`

---

### Beispiel

Sei `$\Omega = \{ K, Z \}$`

`$$p(K) = \frac{1}{3}; p(Z) = \frac{2}{3}$$`

`$$\implies I(K) \approx 1,585; I(Z) = 0,585$$`

---

## Entropie

- Von-Neumann-Entroie {.fragment}
- Thermodynamische Entropie {.fragment}
- Statistische Mechanik {.fragment}
- Reaktionsentropie {.fragment}
- ==Shannon-Entropie== {.fragment}

---

### Definition

Erwartungswert der Information

`$$H(X) = \mathbb{E}_X[I(X)]$$`

`$$H(X) = - \sum_{x \in X}p(x)\log_2p(x)$$` {.fragment}

---

### Bedingte Entropie

<!-- Sei $H(X|Y) = \sum_{x,y}p(y)p(x|y)\log_2{p(x|y)}$ die bedingte Entropie von $X$
unter der Bedingung $Y$. $H(X|Y)$ ist nie größer als $H(X)$ und nur gleich, wenn
$Y$ vollkommen unabhängig von $X$ ist. Das lässt sich anschaulich so erklären,
dass zusätzliche Information ($Y$) nicht Wissen über $X$ zerstören kann, sondern
höchstens vollkommen irrelevant sein kann. -->

`$$H(X|Y) = \sum_{x,y}p(y)p(x|y)\log_2{p(x|y)}$$`

Entropie von `$X$` mit der gegebenen Information über `$Y$`

---

### Relative Entropie

Maß für Unterschiedlichkeit zweier Wahrscheinlichkeitsverteilungen $p$ und $r$

`$$D(p||r) = \sum_x p(x)\log_2{\frac{p(x)}{r(x)}}$$`

`$$D(p||r) \ge 0$$`

Keine Metrik

---

## Markov-Prozesse

---

### Definition

---

### Stationäre Verteilung

---

## Zweiter Hauptsatz der Thermodynamik

---

### $H(X_n)$ steigt monoton

---

### $H(X_0|X_{-\infty}^{-n})$ steigt monoton