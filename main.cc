#include <stdio.h>

#include "data.h"

int main()
{
    Data *data = createData(99);


    doSomething(data);

    destroyData(data);
    return 0;
}
