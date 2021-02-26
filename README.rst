Algoritmos de Ordenação
************************

BubbleSort
-----------
- No melhor caso, o algoritmo executa n operações relevantes, onde n representa o número de elementos do vector. 
- No pior caso, são feitas n^2 operações. 
- A complexidade desse algoritmo é de ordem quadrática. Por isso, ele não é recomendado para programas que precisem de velocidade e operem com quantidade elevada de dados. 

SelectionSort
--------------
- Por não usar um vetor auxiliar para realizar a ordenação, ele ocupa menos memória. 
- Ele é uns dos mais velozes na ordenação de vetores de tamanhos pequenos.
- Ele faz sempre (n^2 − n)/2 comparações, independentemente do vetor estar ordenado ou não.

InsertionSort
--------------
- É o método a ser utilizado quando o arquivo está "quase" ordenado
- É um bom método quando se desejar adicionar poucos elementos em um arquivo já ordenado, pois seu custo é linear.
- O algoritmo de ordenação por inserção é estável.
- O(n^2) sendo ótimo quando a lista está quase ordenada.
- Alto custo de movimentação de elementos no vetor.

QuickSort
----------
- O pior caso de particionamento ocorre quando o elemento pivô divide a lista de forma desbalanceada (quando a lista já está ordenada, ou inversamente ordenada).
- O melhor caso de particionamento acontece quando ele produz duas listas de tamanho não maior que n/2.
- Complexidade de espaço: O(log2n) no melhor caso e no caso médio e O(n) no pior caso
