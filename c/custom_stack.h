struct node
{
	el_type element;         //���������, ���������� ������� �����
	struct node *next;       //��������� �� ��������� ����
};
     
typedef struct node node;    //��� �������� ����� �������� ����������� ��� struct node

node *push(node *stack, el_type element);
el_type pop(node **pstack);
