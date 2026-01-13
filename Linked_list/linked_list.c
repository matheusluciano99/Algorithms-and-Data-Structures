#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    int data;
    struct Node* next;
} Node;

Node* create_node(int data) {
    Node* new_node = (Node*)malloc(sizeof(Node));
    if (!new_node) {
        fprintf(stderr, "Memory allocation failed\n");
        exit(EXIT_FAILURE);
    }
    new_node->data = data;
    new_node->next = NULL;
    return new_node;
    
}

Node* insert_beginning(Node* head, int data) {
    Node* new_node = create_node(data);
    new_node->next = head;
    return new_node;
}

void print_list(Node* head) {
    Node* current = head;
    while(current != NULL){
        printf("%d -> ", current->data);
        current = current->next;
    }
    printf("NULL\n");
}

int main() {
    Node* lista = NULL;  // lista vazia
    
    lista = insert_beginning(lista, 3);
    lista = insert_beginning(lista, 8);
    lista = insert_beginning(lista, 5);
    
    print_list(lista);  // 5 -> 8 -> 3 -> NULL
    
    return 0;
}
