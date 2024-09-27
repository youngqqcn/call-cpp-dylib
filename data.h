#ifndef DATA_H
#define DATA_H

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct Data Data; // Forward declaration
    Data *createData(int n);
    void destroyData(Data *data);
    void doSomething(Data *data);

#ifdef __cplusplus
}
#endif

#endif // DATA_H
