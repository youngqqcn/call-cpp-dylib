#include "data.h"
#include <stdlib.h>
#include "mydata.h"

Data *createData(int n)
{
    Data *data = (Data *)(new MyData(n));
    return data;
}

void destroyData(Data *data)
{
    if (NULL != data)
    {
        delete (MyData *)data;
        data = NULL;
    }
}

void doSomething(Data *data)
{
    ((MyData *)data)->display();
}
