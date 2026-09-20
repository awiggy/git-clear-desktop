import CoreGraphics
import Foundation
let args = CommandLine.arguments
let x = Double(args[1])!, y = Double(args[2])!
let move = CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: CGPoint(x: x, y: y), mouseButton: .left)
move?.post(tap: .cghidEventTap)
print("moved")
