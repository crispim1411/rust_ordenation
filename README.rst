Algoritmos de Ordenação
************************

BubbleSort: 
-----------
Complexidade: O(n2) 

- não é recomendado para programas que precisem de velocidade e operem com quantidade elevada de dados. 

SelectionSort
--------------
Complexidade: O(n2)

- Por não usar um vetor auxiliar para realizar a ordenação, ele ocupa menos memória. 
- Ele é uns dos mais velozes na ordenação de vetores de tamanhos pequenos.

InsertionSort
--------------
Complexidade: O(n2) 

- É o método a ser utilizado quando o arquivo está "quase" ordenado
- É um bom método quando se desejar adicionar poucos elementos em um arquivo já ordenado, pois seu custo é linear.
- Alto custo de movimentação de elementos no vetor.

QuickSort
----------
Complexidade: O(nlogn)

- O pior caso de particionamento ocorre quando o elemento pivô divide a lista de forma desbalanceada (quando a lista já está ordenada, ou inversamente ordenada).
- O melhor caso de particionamento acontece quando ele produz duas listas de tamanho não maior que n/2.

MergeSort
----------
Complexidade: O(nlogn)

- Eficiente para listas maiores 
- Maior consumo de memória
- sempre O(nlogn)
