## Парсеры и комбинаторы

**Парсеры** принимают какие-то параметры (возможно функции но не парсеры) и на их основе создают парсер который принимает ввод `input` и возвращает результат если парсинг успешен в виде кортежа из двух частей: новый ввод и результат парсинга `(new_input, result)`, либо ошибку если парсинг неуспешен. 

**Комбинаторы парсеров** принимаю парсеры и возвращают новые парсеры.

### Парсеры

- #### парсеры которые парсят один раз и возвращают результат

```rust
fn data_end(a:&'a[T]) -> Result<(&[T],&[T]), PErr<'a,T>>
```
парсер который проверяет достигнут ли конец данных, возвращает `Ok` если достигут конец данных иначе ошибку.

```rust
fn any(pattern: &'a[T]) -> impl Parser<'a,T,&'a[T]> 
```
принимает набор допустимых символов и парсит один символ проверяя есть ли он среди допустимых, возвращает один символ либо ошибку.

```rust
fn starts_with(pattern: &'a[T]) -> impl Parser<'a,T,&'a[T]>
```
парсер проверяет начинается ли контекст с заданной последовательности символов, возвращает заданную последовательность отделенную от контекста. 

```rust
fn starts_with_any(patterns: &'a[&'a[T]]) -> impl Parser<'a,T,&'a[T]>
```
он как `starts_with` только принимает не одну а набор последовательностей.

```rust
fn take(count:usize) -> impl Parser<'a,T,&'a[T]>
```
парсер который ни чего не проверяет а тупо откусывает заданное количество элементов.

- #### парсеры последовательности, которые парсят входные данные не один раз а до тех пор пока им парсится, хоть до конца текста, возвращают результат когда остановятся достигнув определенных условий.

```rust
fn seq_max(p: P, count_max:usize) -> impl Parser<'a,T,&'a[T]>
where 
	P: Fn(& T) -> bool
```
последовательность элементов удовлетворяющих предикату, длина последовательности ограничена с верху.

```rust
fn seq_min(p: P, count_min:usize) -> impl Parser<'a,T,&'a[T]>
where
	P: Fn(& T) -> bool
```
как `seq_max` но ограничение не с вреху а с низу.

```rust
fn seq_range(p: P, range:(usize,usize)) -> impl Parser<'a,T,&'a[T]>
where
	P: Fn(& T) -> bool
```
тоже самое но ограничение и с низу и с верху

```rust
fn seq_exact(p: P, count_exact:usize) -> impl Parser<'a,T,&'a[T]>
where
	P: Fn(& T) -> bool
```
тоже самое но парсит точно заданное кол-во элементов последовательности

```rust
fn seq(p: P) -> impl Parser<'a,T,&'a[T]>
where
	P: Fn(& T) -> bool
```
тоже самое но какие либо ограничения на количество отсутствуют.

```rust
fn seq_ext(p: P) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& [T]) -> usize
```
extendet variant sequence, тоже самое, но функция это не логический предикат об одном элементе а числовой предикат о входных данных в целом.

```rust
let mut value: ClassOfSymbols<u8> = Default::default();
```
`ClassOfSymbols` это декларативный подход к построению парсера последовательности. Объект в котором мы описываем какие элементы разрешены а какие запрещены. Делаем это с помощью методов объекта:
```rust
one_enable_push(&mut self, p:&[I])        -> &mut Self 
one_disable_push(&mut self, p:&[I])       -> &mut Self
range_enable_push(&mut self, p:&[(I,I)])  -> &mut Self
range_disable_push(&mut self, p:&[(I,I)]) -> &mut Self
parts_enable_push(&mut self, p:&[&[I]])   -> &mut Self
parts_disable_push(&mut self, p:&[&[I]])  -> &mut Self
default_enable_one(&mut self, b:bool)     -> &mut Self
```
если вы используете напрямую парсер `ClassOfSymbols` указывайте его по ссылке, как здесь
```rust
let name_parser  = between_opt(space, &name, space);
```
если используете в dot нотации, то ссыку указывать не надо методы сами берут его по ссылке
```rust
let name_parser  = between_opt(space, name.msg_err("pars name error!"), space);
``` 

Зачастую гибкость в определении не нужна, а нужна возможность создать `static` or `const` значение, для этого используйте `StaticClassOfSymbols`,
это тоже самое что и `ClassOfSymbols` только `static`.

```rust
static t: StaticClassOfSymbols<u8> = StaticClassOfSymbols::new();
```
и названия методов схожи 
```rust
const fn new() -> Self
const fn one_enable_set(mut self, p:&'static[I]) -> Self
const fn one_disable_set(mut self, p:&'static[I]) -> Self 
const fn parts_enable_set(mut self, p:&'static[&'static[I]]) -> Self 
const fn parts_disable_set(mut self, p:&'static[&'static[I]]) -> Self 
const fn range_enable_set(mut self, p:&'static[(I,I)]) -> Self 
const fn range_disable_set(mut self, p:&'static[(I,I)]) -> Self 
const fn default_enable_one(mut self, p:bool) -> Self
```





### Парсер комбинаторы

...


### Обработка ошибок

если вам нужно провести отладочную трассировку либо просто добовить сообщение об ошибке к некоторому парсеру используйте парсер комбинатор `msg_err`
```rust
fn msg_err(parser: P, msg: &'a str) -> impl Parser<'a,T,R>
where
    P: Parser<'a,T,R>
```
если вы парсите `&str` но парсите как `&[u8]` будет одна неприятность, при ошибке вам будет выводится участок данных на котором произошла ошибка парсинга в виде байтовой последовательности, чтобы побороть эту неприятность в конце цепочки перед самым вызовом `.parse(input)?;` используйте парсер комбинатор `.strerr()` как здесь: 
```rust
let (input, (tag_name, tag_attrs)) = between(open, pair(name_parser, attrs), close)
	.msg_err("first line pars eror")
	.strerr()
	.parse(input)?;
```
для проверки причины ошибки - был ли достигнут конец данных, у ошибки парсинга есть метод:
```rust
e.is_eod() -> bool
```