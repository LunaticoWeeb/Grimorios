# CERN ROOT

## Histograma

Definir uma função com o mesmo nome do arquivo (graph.c):

```c
void graph() 
{
    // ...
}
```

Assim define o tipo do histograma:

```c
TH1F *h = new TH1F("h", "Histogram", 100, 0, 6);
/*
    TH = Tipo Histograma
    1 = 1 dimensão
    F = Float (Inteiro = I)
    h = Nome do histograma
    Histogram = Título do histograma
    100 = Número de bins
    0 = Valor mínimo
    6 = Valor máximo
*/
```

Definimos o quadro do histograma:

```c
TCanvas *c1 = new TCanvas();
h->Draw();
c1->SaveAs("histogram.png");
/*
    TCanvas = Quadro
    c1 = Nome do quadro
    h->Draw() = Desenhar o histograma
    c1->SaveAs("histogram.png") = Salvar o histograma como imagem
*/
```

Executando o programa:
```bash
root graph.c
```

Adicionando valores ao histograma:

```c
h->Fill(2.2);
h->Fill(2.2);
h->Fill(3.2);
h->Fill(4.2);
```

Título do eixo x e y:
```c
h->GetXaxis()->SetTitle("x");
h->GetYaxis()->SetTitle("y");
```

### Lendo dados de um arquivo
Exemplo de arquivo de dados (data.txt):

```txt
1.1
3.4
2.2
4.5
1.2
3.5
2.3
4.6
```

Abrindo o arquivo:

```c
ifstream file("data.txt");
```

Leitura:
```c
float x;
while (file >> x)
{
    hist->Fill(x);
}
```

Fechar o arquivo:
```c
file.close();
```

## Histograma 2D

```c
void hist2d()
{
      TCanvas *c1 = new TCanvas(); // Criar um quadro
      gStyle->SetPalette(kDarkBodyRadiator); // Definir a paleta de cores (https://root.cern/doc/master/classTColor.html#C06)
      TH2F *h = new TH2F("h", "Histogram", 50, 3, 6, 25, 0, 10);
      /*
            TH = Tipo Histograma
            2 = 2 dimensões
            F = Float (Inteiro = I)
            h = Nome do histograma
            Histogram = Título do histograma
            50 = Número de bins em x
            3 = Valor mínimo em x
            6 = Valor máximo em x
            25 = Número de bins em y
            0 = Valor mínimo em y
            10 = Valor máximo em y
      */
     h->Fill(4.2, 2.2); // Preencher o histograma com os valores de x e y

     h->GetXaxis()->SetTitle("x"); // Definir o título do eixo x
     h->GetYaxis()->SetTitle("y"); // Definir o título do eixo y
     h->GetZaxis()->SetTitle("z"); // Definir o título do eixo z

     h->SetContour(100); // Definir o número de cores
     hist->Draw("colz"); // Desenhar o histograma, colz = colorido
}
```


## Gráficos

```c
void graph()
{
      double x[] = {1, 2, 3, 4, 5, 6, 7, 8, 9}; // Valores de x
      double y[] = {1, 2, 3, 4, 5, 6, 7, 8, 9}; // Valores de y

      TGraph *gr = new TGraph(9, x, y); // Criar um gráfico
      /*
            TGraph = Tipo de gráfico
            9 = Número de pontos
            x = Valores de x
            y = Valores de y
      */

     gr->SetMarkerStyle(20); // Definir o estilo do marcador
     gr->SetMarkerSize(1); // Definir o tamanho do marcador
     gr->SetMarkerColor(2); // Definir a cor do marcador

     gr->GetXaxis()->SetTitle("x"); // Definir o título do eixo x
     gr->GetYaxis()->SetTitle("y"); // Definir o título do eixo y

     gr->SetTitle("Graph"); // Definir o título do gráfico

     gr->GetXaxis()->SetLimits(0, 10); // Definir os limites do eixo x

     // Definir os limites do eixo y
     gr->SetMinimum(0);
     gr->SetMaximum(10); 

     TCanvas *c1 = new TCanvas(); // Criar um quadro
     gr->Draw("ACP"); // Desenhar o gráfico, AC* = pontos e linhas
      /*
            A = Eixos
            C = Curva
            * = Estrela
            L = Linha
            P = Ponto
      */
}
```

Leitura e entrada dos dados:

```c
void grafico()
{

      int x, y, i = 0; // Valores de x e y e contador
      
      TGraph *g = new TGraph();

      ifstream data("data.txt");

      while(data >> x >> y)
      {
            g->SetPoint(i, x, y); 
            i++;
      }

      data.close();

      g->SetMarkerStyle(20);
      g->SetMarkerSize(1);
      g->SetMarkerColor(2);
      g->SetTitle("Grafico");
      g->GetXaxis()->SetTitle("x");
      g->GetYaxis()->SetTitle("y");

      TCanvas *c = new TCanvas();
      g->Draw("AP");
}
```
