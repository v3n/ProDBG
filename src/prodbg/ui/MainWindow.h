#pragma once

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include <QMainWindow>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

QT_BEGIN_NAMESPACE
class QMenu;
class QSignalMapper;
class QTimer;
QT_END_NAMESPACE

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

namespace prodbg
{

struct Plugin;
struct Session;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PluginInfo
{
	Plugin* plugin;
	int menuItem;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct MenuDescriptor;

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow();
    virtual ~MainWindow();

private slots:
    void onMenu(int id);
	void tick();

private:

	void createMenu(MenuDescriptor* desc, QMenu* menu);
	void createMenuItem(MenuDescriptor* desc, QMenu* menu);
	void createWindowMenu();
	void onAttachRemote();

	QMenu* m_pluginMenu;
	QTimer* m_timer;

    QSignalMapper* m_signalMapper;

    PluginInfo* m_pluginInfoArray;
    Session* m_remoteSession;
    Session* m_activeSession;
    int m_pluginCount;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

}

