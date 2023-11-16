# Algebraic Mode Commands

## To find/create points
| Sequence                        | Description                                                                |
| :------------------------------ | -------------------------------------------------------------------------: |
| g + "x" + "y" + "z"             | Create a point given its components x, y and z.                            |
| g + (point1) + (point2)         | Find the reflection of point1 about point2                                 |
| g + (point) + . + (point)       | Find the midpoint between given points.                                    |
| g + (point) + (line)            | Find the point that is the reflection of given point about given line.     |
| g + (line) + (plane)            | Find the point of intersection between a line and a plane.                 |
| g + (line) + (line)             | Find the point of intersection between the given concurrent lines.         |
| g + (plane) + (plane) + (plane) | Find the point of intersection between three planes.   |
| g + (point) + "d" + (line)      | Find the 2 points at a distance d from given point that lie in given line. |


# To find/create a lines
| Sequence                        | Description                                                |
| :------------------------------ | ---------------------------------------------------------: |
| r + (point) + (point) | Find the line that passes through the two given points. |
| r + (point) + . + (point) | Find the perpendicular bisector of the given points. |
| r + (point) + (line)  | Find the line parallel to given line that contains given point|
| r + (point) + . + (line)  | Find the line orthogonal (and concurrent) to given line that contains given point|
| r + (point) + (plane) | Find the line orthogonal to given plane that passes through given point. |
| r + (line) + (line) | Find the angle bisector between the given concurrent lines. |
| r + (plane) + (plane) | Find the line of intersection between the given planes. |

# To find/create a plane
| Sequence                        | Description                                                        |
| :------------------------------ | -----------------------------------------------------------------: |
| v + (point) + (point)           | Find the bisecting plane between the given points.                 |
| v + (point) + (point) + (point) | Find the plane that contains the given points.                     |
| v + (point) + (line)            | Find the plane that contains given line and given point.           |
| v + (point) + . + (line)        | Find the plane orthogonal to given line that contains given point. |
| v + (line) + (line)             | Find the plane that contains the given concurrent lines.           |
| v + (line) + . + (line)         | Find the bisecting plane between the given concurrent lines.  |
| v + (plane) + (plane)           | Find the bisecting plane between the given concurrent planes       |
|                                 |                                                                    |

# Other commands
| Sequence                        | Description                                                |
| :------------------------------ | ---------------------------------------------------------: |
| d + (object) | Delete object |
| h + (object) | Hide object |
| u | undo |
| Ctrl + r | redo |




