# 第 15 章：智能指针：怎么拥有和访问数据

Box 管理独占的堆数据，Rc 管理单线程共享所有权，RefCell 在运行时检查借用，Weak 表示不延长目标寿命的回指。先确定要解决哪种问题，再选类型。

- [Box：一个所有者，管理堆上的数据](ch15-01-box.md)
- [Deref：让包装类型能像引用一样使用](ch15-02-deref.md)
- [Drop：值销毁时做清理](ch15-03-drop.md)
- [Rc：让多个所有者共享一份数据](ch15-04-rc.md)
- [RefCell：把借用检查放到运行时](ch15-05-interior-mutability.md)
- [Weak：别让相互引用变成泄漏](ch15-06-reference-cycles.md)

[返回目录](SUMMARY.md)
