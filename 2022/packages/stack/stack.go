package stack

import "fmt"

type Stack[T any] struct {
	top    *Node[T]
	Length int
}

func New[T any]() Stack[T] {
	return Stack[T]{
		Length: 0,
	}
}

type Node[T any] struct {
	Value T
	next  *Node[T]
}

func (s *Stack[T]) Push(val T) {
	s.push(&Node[T]{Value: val})
}

func (s *Stack[T]) push(node *Node[T]) {
	if s.top == nil {
		s.top = node
	} else {
		node := node
		temp := s.top

		node.next = temp
		s.top = node
		s.Length += 1
	}
}

func (s *Stack[T]) pop() *Node[T] {
	if s.top != nil {
		node := s.top
		s.top = node.next
		s.Length -= 1

		return node
	}

	return nil
}

func (s *Stack[T]) Pop() *T {
	node := s.pop()

	if node != nil {
		return &node.Value
	} else {
		return nil
	}
}

func (s *Stack[T]) Print() {
	if s.top == nil {
		return
	}

	top := s.top

	s.pop()

	s.Print()

	fmt.Printf("%v ", top.Value)

	s.push(top)
}
