# Ray Tracing in One Week Notes
[the book](https://raytracing.github.io/)
[Ray Tracing in One Weekend 超详解](https://www.cnblogs.com/lv-anchoret/p/10163205.html)
用Rust实现，记录对the book的学习理解进程，与对Rust特性的学习探索有感

## Output an Image
了解PPM格式
```text
P3 # fixed
256 256 # weigh height
256 # colors
r g b
r g b
# left to right, and then up to down
......
```
![](./imgs/2023-01-04-22-34-13.png)

## The vec3 Class
被Rust运算符重载折磨了一下，相比C++只需重载一个函数，Rust需要实现一个Trait，多包了一层有亿点冗长。然后左右不同类型也是要实现两次，`XXAssign`也要重新实现，很麻烦但貌似确实没啥太好的办法。(当时刚学C++运算符重载时也是一脸懵逼)

![](./imgs/2023-01-05-08-54-46.png "要是能像这样打包一下，不用写那么多trait也好哇")

相比原文的`using point3=vec3;using color=vec3;`，Rust适合使用**new type**模式，用`tuple struct`而非`type`，实现强类型约束，不会犯把一个颜色加到一个坐标上的错误
* 经过实践，发现这样的类型约束也有那么点鸡肋的感觉，要是我习惯加.0了那不就完全不起作用了吗🤣。

## Ray, a Simple Camera, and Background
### Ray Class
起点和方向组成一束光线(t为正数时为射线)

### 在场景中发射光线
光线追踪器发送光，并计算这些光路方向上的颜色。需要以下几步：
1. 计算从眼睛到像素的光线(光路可逆)，将射线从视点发射到像素坐标
2. 判断哪个物体与光线相交，相交位置
3. 计算出相交点上的颜色

发射时射线向量并不需要是单位向量

除了设置渲染图像的像素尺寸外，我们还需要设置一个**虚拟视口**来**传递我们的场景光线**。对于标准的正方形像素间距，视口的纵横比应该与我们渲染的图像相同。我们只选择一个高度为两个单位(-1~+1)的视口。我们还**将投影平面和投影点之间的距离设置为一个单位**。这被称为“焦长度”(`focal_length`)，不是焦距！

![](./imgs/2023-01-05-10-18-27.png)

左手系,相机在(0,0,0),x正方向为右，y正方向为上，场景的z为负数

了解了一下C++的`constexpr`函数与Rust的`const fn`。它们可以用来初始化常量；条件是保证只要所有参数都是常量表达式，返回值就一定是(编译期能计算出来的)常量表达式。
* Rust的`Trait Implement`中无法使用`const fn`，但有实验性的`impl const XXTrait for XXStruct{}`语法。
* Rust的`const fn`中进行浮点数运算是实验性的，因为考虑到跨平台问题，不能保证编译期与运行期的运算结果完全一致(虽然有IEEE，但不同平台`NaN behavior`不同)
  >[Must a const fn behave exactly the same at runtime as at compile-time?](https://github.com/rust-lang/rust/issues/77745)
* 敬佩Rust社区的严谨态度，但我没有时间深究这些`issues`, `unstable`和`experimental`了，最后还是放弃了使用过多的编译期计算

线性混合公式(Blend)：$$blendedValue=(1−t)\cdot startValue+t\cdot endValue$$
(其实就是二维线性插值)

`ray_color()`利用混合公式线性地将白色与天蓝色混合

主要以下几步:

1. 确定分辨率: 假定宽度400,比例16:9
2. 确定坐标映射系统，场景左下角为(-2,-1,-1)，右上角为(2,1,-1)，中点为(0,0,-1)；摄影机(0,0,0)
3. 按像素依次发射光线、计算、混合

效果：
![](./imgs/2023-01-05-17-38-17.png)

### Adding a Sphere
终于要添加真正的物体了！

计算光线有没有击中一个球体很简单，所以一般都用球体测试光追

#### 判断光与球相交
设P为一个坐标点，C为球心坐标$\vec P=(x,y,z), \vec C =(C_x,C_y,C_z)$，则P在C球面上的公式为：
$$(\vec P−\vec C)⋅(\vec P−\vec C)=r^2$$
若P为光线上的点$\vec P(t)=\vec A+t\vec b$,则：
$$(\vec P(t )−\vec C)⋅(\vec P(t)−\vec C)=r^2$$
或者把光线函数扩展为点向式:
$$(\vec A+t\vec b−\vec C)⋅(\vec A+t\vec b−\vec C)=r^2$$
整理得到:
$$\vec b\cdot \vec b \cdot t^2+2\vec b\cdot(\vec A−\vec C) \cdot t+(\vec A−\vec C)⋅(\vec A−\vec C)−r^2=0$$
是关于t的二次方程

![](./imgs/2023-01-05-18-08-52.png)

#### 完成第一幅光追图像
没有着色，也没有反射的球图像
![](imgs/2023-01-05-19-25-51.png)

还有个bug(特性😅)是判断球时t的解可以为负数，所以如果把球z坐标改成-1你仍然可以看到你背后的球

### Surface Normals and Multiple Objects
#### 用表面法线着色
法线是在相交点上与表面垂直的向量。长度方面，是单位向量的话比较方便着色。方向方面，对球来说向外的法线是交点减去球心

可视化法线的常用技巧是直接把法线方向映射到RGB上。它对单位向量法线来说很简单直观。

![](imgs/2023-01-05-21-14-43.png)

#### 优化公式

#### 抽象出与光线交互的物体Object

