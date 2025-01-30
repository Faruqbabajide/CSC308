#include <stdio.h>

int main(){
    
    char name[6]= "George";

    printf("%p",name);
    for (int i= 0; i< 6; i++){
        printf("%c\n", name[i]);
    }


}