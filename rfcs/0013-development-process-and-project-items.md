# Overview

This RFC proposes specific guidelines for project item design and communication. 

These guidelines ensure all development teams are aligned, and informed, settle the right expectations and work in nice synchrony to deliver company goals.

No specific hierarchy items, tags or team organizations are expected.

This RFC assumes there could be out-of-items or out-of-process work. 
So it is considered optional and decided on per team and collaboration basis.

Expectancy that all cross-team collaboration will adhere to the constraints outlined in this guide. 

In a team, collaboration would adhere to this guide too, until 100% full consensus to avoid that with caution. Any team member may reject work that is not aligned with the flow outlined here.

RFC does not specififies what happens in some edge cases, example in case of vacations, assuming commons sence is used in these case. 

## Problem

Currently, development produces wasted output, misses people's expectations on communication channels, and does not align the expected output of one team with another.  And doing all this in timely manner. 

## Solution

All work items MUST are entered in ClickUp[CU]
People tagged or mentioned in CI or watching relevant tasks are expected to respond within 24 working hours. 
GitHub (GH) issues, GH metions or review requests notifications are good help, but not required. 
If there is dependency on exteranal our of companu CU work item work or dependcy, it should be accompanied by in CU referencing work item
If person is not working on CU item during 24 hours (or not going to do so) it changes status to ensure it is NOT  `in progress` item. You should leave comment if you plan to proceed later.
When task is started to work on, it set to equivalent of `progress state`.
If you stop any work on item and not going to do any progress, you unassign task from you. Settle expectations by setting some final end status  or set some intermediate status assigned with assiganation other people or do 3, but remove you from assigned and add comentary why you are doing so.
If task assigned to you and it is not clear why it was happend, you have 24 working hours to ask for clarification.
If any item takes more then several days to deliver, split it into subtask with resonable delivery goals.
If task grows large because of unvoered new items, limit scope of this task deliver and outline new tasks which scope remaining work. Explain why this is done.
Any work which is not considered to be done or planned should not be in CU. 
If any work was considered to be and decided not to be later, it should be set to end state by current assigen or its manager.
Repeating, work which is not considered to be done should not be click up item.
CU creation can be dicussed. If dicussion is long and inteream, task to align/decide should be created.
Non of tasks could created or watched by single person. If task was created you should make watch or tag with explanation people who should be aware of its existence and progress.
Task created by single person and not tagging any other people should not exists. Consider that to be `at least 1 reviewer` rule for task.  
If task was created as result of communiction in GH or Slack, refere GH item or Slack dicussion (or people from diucssion) in CU item.
If work item is assigned to several people and in progress. Specific subitem it outlined assigned to one person.

## Notes

If you want to learn CU, please check [this](https://univerasity.clickup.com/page/course-catalog#level_novice,role_member).

This guidlines compose nicely with per item definition of done or/and acceptance criteria.


