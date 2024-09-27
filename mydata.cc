#include "mydata.h"
#include <stdio.h>

MyData::MyData(int v)
{
    value = v;
}

MyData::~MyData()
{
    printf("deleted\n");
}

void MyData::display()
{
    printf("value: %d\n", value);
}


