#include "pgraph.h"

int main()
{
	image *img = create_image(401, 301);		//�������� �����������
	
	set_cur_col(img, (color) {153, 217, 234});	//������� ���� - ������-�������
	
	for (uint i = 0; i <= 400; i += 10)			//������������
		line(img, i, 0, i, 300);				//�����
	
	for (uint i = 0; i <= 300; i += 10)			//��������������
		line(img, 0, i, 400, i);				//�����
	
	set_cur_col(img, BLACK);					//������� ���� - ������
	
	line(img, 0, 150, 400, 150);				//��� �������
	
	line_to(set_cur_pnt(img, 390, 155), 400, 150);  //C������ ��
	line_to(img, 390, 145);						//��� �������
	
	line(img, 200, 0, 200, 300);				//��� �������
	
	line_to(set_cur_pnt(img, 195, 290), 200, 300);  //C������ ��
	line_to(img, 205, 290);						//��� �������
	
    set_cur_col(img, RED);						//������� ���� - �������
    
    //���������� ���������
	set_cur_pnt(img, 0, (int) round (150 + 50 * sin (-4)));
    for (int x = 0; x < 401; x++)
    {
    	int y = round (150 +   50 * sin ((x-200) / 50.0));
    	line_to(img, x, y);
	}
    
    //���������� ����������
    double delta = 3.141592653589793 / 200;
    set_cur_col(img, GREEN);
    set_cur_pnt(img, 300, 150);
    for (int n = 0; n <= 400; n++)
    {
    	int x = round(200 + 100 * cos (n * delta));
    	int y = round(150 + 100 * sin (n * delta));
    	line_to(img, x, y);
	}

	fill(img, 220, 160, WHITE);				//�������� �������
	fill(img, 210, 155, WHITE);				//�������� �������� �������
	fill(img, 220, 160, SILVER);			//������� ������� ����� ������

	if (!save_to_file(img, "test.bmp"))		//������ ����������� � ����
		puts("�� ������� ��������� ����");
	
	free(img);								//�������� �����������
	
	return 0;
}
